use rdkafka::{message::ToBytes, producer::{FutureProducer, FutureRecord}};
pub struct Api <'a ,T, B> where T: ToBytes + ?Sized,
B: ToBytes + ?Sized 
{
    future_producer: FutureProducer,
    future_record: FutureRecord<'a, T, B>,
}