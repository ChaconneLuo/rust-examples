pub trait Message {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Message> {
    pub messenger: &'a T,
    pub value: usize,
    pub max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Message,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn send(&self, msg: &str) {
        self.messenger.send(msg);
    }
}
