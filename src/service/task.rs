use pipeline::{Request, Time};

trait IsEnoughFor<Process> {
    fn is_enough_for(&self, process: &Process) -> bool;
}

#[derive(Debug, Clone, Copy)]
pub(super) struct Task {
    pub request: Request,
    pub handling_time: Time,
}

#[derive(Debug)]
pub(super) enum HandlingState {
    InProgress {
        task: Task,
        waiting_time: Time,
    },
    Completed {
        request: Request,
        waiting_time: Time,
    },
}

impl Task {
    pub fn handle_during(mut self, delta_time: &mut Time) -> HandlingState {
        if delta_time.is_enough_for(&self) {
            let waiting_time = self.complete_in(delta_time);
            self.request.leaving_time += waiting_time;

            HandlingState::Completed {
                request: self.request,
                waiting_time,
            }
        } else {
            let waiting_time = self.progress_during(delta_time);
            self.request.leaving_time += waiting_time;

            HandlingState::InProgress {
                task: self,
                waiting_time,
            }
        }
    }

    fn complete_in(&mut self, delta_time: &mut Time) -> Time {
        let waiting_time = self.handling_time;

        *delta_time -= self.handling_time;
        self.handling_time = Time::ZERO;

        waiting_time
    }

    fn progress_during(&mut self, delta_time: &mut Time) -> Time {
        let waiting_time = *delta_time;

        self.handling_time -= *delta_time;
        *delta_time = Time::ZERO;

        waiting_time
    }
}

impl IsEnoughFor<Task> for Time {
    fn is_enough_for(&self, process: &Task) -> bool {
        *self >= process.handling_time
    }
}
