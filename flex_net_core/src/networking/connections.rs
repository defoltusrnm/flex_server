pub trait NetWriter
where
    Self: Send + Sized,
{
    fn write();
}

pub trait NetReader
where
    Self: Send + Sized,
{
    fn read();
}

pub trait NetConnection: NetReader + NetWriter {}
