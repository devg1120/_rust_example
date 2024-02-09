import fetch from "node-fetch";

import * as vscode from "vscode";
import * as stream from "stream";
import * as crypto from "crypto";
import * as fs from "fs";
import * as zlib from "zlib";
import * as util from "util";
import * as path from "path";
import { log, assert } from "./util";
import * as https from "https";

const pipeline = util.promisify(stream.pipeline);

const GITHUB_API_ENDPOINT_URL = "https://api.github.com";
const OWNER = "rune-rs";
const REPO = "rune";

export async function fetchRelease(
    releaseTag: string,
    githubToken: string | null | undefined,
): Promise<GithubRelease> {

    const apiEndpointPath = `/repos/${OWNER}/${REPO}/releases/tags/${releaseTag}`;

    const requestUrl = GITHUB_API_ENDPOINT_URL + apiEndpointPath;

    log.debug("Issuing request for released artifacts metadata to", requestUrl);

    // eslint-disable-next-line @typescript-eslint/naming-convention
    const headers: Record<string, string> = { "Accept": "application/vnd.github.v3+json" };
    if (githubToken !== null) {
        headers.Authorization = "token " + githubToken;
    }

    const response = await (() => {
        const agent = new https.Agent();
        return fetch(requestUrl, { headers: headers, agent: agent });
    })();

    if (!response.ok) {
        log.error("Error fetching artifact release info", {
            requestUrl,
            releaseTag,
            response: {
                headers: response.headers,
                status: response.status,
                body: await response.text(),
            }
        });

        throw new Error(
            `Got response ${response.status} when trying to fetch ` +
            `release info for ${releaseTag} release`
        );
    }

    // We skip runtime type checks for simplicity (here we cast from `unknown` to `GithubRelease`)
    const release = await response.json() as GithubRelease;
    return release;
}

// We omit declaration of tremendous amount of fields that we are not using here
export interface GithubRelease {
    name: string;
    id: number;
    // eslint-disable-next-line camelcase, @typescript-eslint/naming-convention
    published_at: string;
    assets: Array<{
        name: string;
        id: number;
        // eslint-disable-next-line camelcase, @typescript-eslint/naming-convention
        browser_download_url: vscode.Uri;
    }>;
}

interface DownloadOpts {
    progressTitle: string;
    url: vscode.Uri;
    dest: vscode.Uri;
    mode?: number;
    gunzip?: boolean;
}

export async function download(opts: DownloadOpts) {
    // Put artifact into a temporary file (in the same dir for simplicity)
    // to prevent partially downloaded files when user kills vscode
    // This also avoids overwriting running executables
    const randomHex = crypto.randomBytes(5).toString("hex");
    const rawDest = path.parse(opts.dest.fsPath);
    const oldServerPath = vscode.Uri.joinPath(vscode.Uri.file(rawDest.dir), `${rawDest.name}-stale-${randomHex}${rawDest.ext}`);
    const tempFilePath = vscode.Uri.joinPath(vscode.Uri.file(rawDest.dir), `${rawDest.name}${randomHex}`);

    await vscode.window.withProgress(
        {
            location: vscode.ProgressLocation.Notification,
            cancellable: false,
            title: opts.progressTitle
        },
        async (progress, _cancellationToken) => {
            let lastPercentage = 0;
            await downloadFile(opts.url, tempFilePath, opts.mode, !!opts.gunzip, (readBytes, totalBytes) => {
                const newPercentage = Math.round((readBytes / totalBytes) * 100);
                if (newPercentage !== lastPercentage) {
                    progress.report({
                        message: `${newPercentage.toFixed(0)}%`,
                        increment: newPercentage - lastPercentage
                    });

                    lastPercentage = newPercentage;
                }
            });
        }
    );

    // Try to rename a running server to avoid EPERM on Windows
    // NB: this can lead to issues if a running Code instance tries to restart the server.
    try {
        await vscode.workspace.fs.rename(opts.dest, oldServerPath, { overwrite: true });
        log.info(`Renamed old server binary ${opts.dest.fsPath} to ${oldServerPath.fsPath}`);
    } catch (err) {
        const fsErr = err as vscode.FileSystemError;

        // This is supposed to return `FileNotFound` (spelled as `EntryNotFound`)
        // but instead `code` is `Unknown` and `name` is `EntryNotFound (FileSystemError) (FileSystemError)`.
        // https://github.com/rust-analyzer/rust-analyzer/pull/10222
        if (!fsErr.code || fsErr.code !== "EntryNotFound" && fsErr.name.indexOf("EntryNotFound") === -1) {
            log.error(`Cannot rename existing server instance: ${err}"`);
        }
    }
    try {
        await vscode.workspace.fs.rename(tempFilePath, opts.dest, { overwrite: true });
    } catch (err) {
        log.error(`Cannot update server binary: ${err}`);
    }

    // Now try to remove any stale server binaries
    const serverDir = vscode.Uri.file(rawDest.dir);
    try {
        const entries = await vscode.workspace.fs.readDirectory(serverDir);
        for (const [entry, _] of entries) {
            try {
                if (entry.includes(`${rawDest.name}-stale-`)) {
                    const uri = vscode.Uri.joinPath(serverDir, entry);
                    try {
                        await vscode.workspace.fs.delete(uri);
                        log.info(`Removed old server binary ${uri.fsPath}`);
                    } catch (err) {
                        log.error(`Unable to remove old server binary ${uri.fsPath}`);
                    }
                }
            } catch (err) {
                log.error(`Unable to parse ${entry}`);
            }
        }
    } catch (err) {
        log.error(`Unable to enumerate contents of ${serverDir.fsPath}`);
    }
}

async function downloadFile(
    url: vscode.Uri,
    destFilePath: vscode.Uri,
    mode: number | undefined,
    gunzip: boolean,
    onProgress: (readBytes: number, totalBytes: number) => void
): Promise<void> {
    const urlString = url.toString();

    const res = await (() => {
        const agent = new https.Agent();
        return fetch(urlString, { agent: agent });
    })();

    if (!res.ok) {
        log.error("Error", res.status, "while downloading file from", urlString);
        log.error({ body: await res.text(), headers: res.headers });

        throw new Error(`Got response ${res.status} when trying to download a file.`);
    }

    if (!res.body) {
        log.error("Empty body while downloading file from", urlString);
        log.error({ headers: res.headers });
        throw new Error(`Got empty body when trying to download a file.`);
    }

    const totalBytes = Number(res.headers.get('content-length'));
    assert(!Number.isNaN(totalBytes), "Sanity check of content-length protocol");

    log.debug("Downloading file of", totalBytes, "bytes size from", urlString, "to", destFilePath.fsPath);

    let readBytes = 0;
    res.body.on("data", (chunk: Buffer) => {
        readBytes += chunk.length;
        onProgress(readBytes, totalBytes);
    });

    const destFileStream = fs.createWriteStream(destFilePath.fsPath, { mode });
    const srcStream = gunzip ? res.body.pipe(zlib.createGunzip()) : res.body;

    await pipeline(srcStream, destFileStream);
}
