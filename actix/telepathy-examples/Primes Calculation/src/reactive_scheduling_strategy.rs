use std::collections::{HashMap, LinkedList};
use actix_telepathy::AnyAddr;
use crate::actors::worker::{Work, Worker};
use std::cmp::min;

const MAX_SUBQUERY_RANGE_SIZE: usize = 100_000;

pub struct ReactiveSchedulingStrategy {
    query_id_2_tracker: HashMap<usize, QueryTracker>,
    worker_2_tracker: HashMap<AnyAddr<Worker>, Option<QueryTracker>>,
}

impl ReactiveSchedulingStrategy {
    pub fn new() -> Self {
        Self {
            query_id_2_tracker: HashMap::new(),
            worker_2_tracker: HashMap::new(),
        }
    }

    pub fn schedule(&mut self, task_id: usize, start: usize, end: usize) {
        let tracker = QueryTracker::new(task_id, start, end);
        self.query_id_2_tracker.insert(task_id, tracker);
        self.assign_subqueries();
    }

    pub fn has_tasks_in_progress(&self) -> bool {
        !self.query_id_2_tracker.is_empty()
    }

    pub fn finished(&mut self, task_id: usize, worker: AnyAddr<Worker>) {
        let query_tracker = self.query_id_2_tracker.get_mut(&task_id).unwrap();
        query_tracker.work_completed(worker.clone());
        self.worker_2_tracker.insert(worker, None);

        if query_tracker.check_complete() {
            self.query_id_2_tracker.remove(&task_id);
        } else {
            self.assign_subqueries();
        }
    }

    pub fn add_worker(&mut self, worker: AnyAddr<Worker>) {
        self.worker_2_tracker.insert(worker, None);
        self.assign_subqueries();
    }

    fn assign_subqueries(&mut self) {
        let idle_workers: Vec<AnyAddr<Worker>> = self.worker_2_tracker.iter()
            .filter_map(|x| {
                match x.1 {
                    Some(_) => None,
                    None => Some(x.0.clone())
                }
            })
            .collect();

        if self.query_id_2_tracker.is_empty() {
            return;
        }

        let mut query_tracker_iterator = self.query_id_2_tracker.iter_mut();
        let mut query_tracker = query_tracker_iterator.next().unwrap().1;
        for idle_worker in idle_workers {
            while !query_tracker.assign_work(idle_worker.clone()) {
                query_tracker = match query_tracker_iterator.next() {
                    Some(tracker) => tracker.1,
                    None => return
                }
            }

            self.worker_2_tracker.insert(idle_worker, Some((*query_tracker).clone()));
        }
    }

    pub fn count_workers(&self) -> usize {
        self.worker_2_tracker.len()
    }
}


struct QueryTracker {
    remaining_range_start: usize,
    remaining_range_end: usize,
    id: usize,
    running_subqueries: HashMap<AnyAddr<Worker>, Work>,
    failed_subqueries: LinkedList<Work>
}

impl QueryTracker {
    pub fn new(id: usize, start: usize, end: usize) -> Self {
        Self {
            remaining_range_start: start,
            remaining_range_end: end,
            id,
            running_subqueries: HashMap::new(),
            failed_subqueries: LinkedList::new()
        }
    }

    pub fn assign_work(&mut self, worker: AnyAddr<Worker>) -> bool {
        let subquery = match self.failed_subqueries.pop_front() {
            Some(work) => work,
            None => {
                let subquery_range_size = min(self.remaining_range_end - (self.remaining_range_start - 1), MAX_SUBQUERY_RANGE_SIZE);
                let mut work: Option<Work> = None;

                if subquery_range_size > 0 {
                    work = Some(Work { id: self.id, start: self.remaining_range_start, end: self.remaining_range_start + subquery_range_size - 1});
                    self.remaining_range_start += subquery_range_size;
                }

                if work.is_none() {
                    return false
                }

                work.unwrap()
            }
        };

        worker.do_send(subquery.clone());
        self.running_subqueries.insert(worker, subquery);

        return true
    }

    pub fn work_completed(&mut self, worker: AnyAddr<Worker>) {
        self.running_subqueries.remove(&worker)
            .expect("There should've been something coming back!");
    }

    pub fn check_complete(&mut self) -> bool {
        self.running_subqueries.is_empty()
            && self.failed_subqueries.is_empty()
            && self.remaining_range_start > self.remaining_range_end
    }
}

impl Clone for QueryTracker {
    fn clone(&self) -> Self {
        Self {
            remaining_range_start: self.remaining_range_start.clone(),
            remaining_range_end: self.remaining_range_end.clone(),
            id: self.id.clone(),
            running_subqueries: self.running_subqueries.clone(),
            failed_subqueries: self.failed_subqueries.clone()
        }
    }
}
