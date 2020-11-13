pub fn read_varint(buf: &[u8]) -> Result<(i32, usize), ()> {
    let mut res: i32 = 0;

    println!("v:{:?}", buf);

    for i in 0..buf.len() {
        res |= ((buf[i] & 0b01111111) as i32) << (7 * i);
        if (buf[i] & 0b1000_0000) == 0 {
            return Ok((res, i + 1));
        }
    }

    Err(())
}

pub fn read_varlong(buf: &[u8]) -> Result<(i64, usize), ()> {
    let mut res: i64 = 0;

    for i in 0..buf.len() {
        res |= ((buf[i] & 0b01111111) as i64) << (7 * i);
        if (buf[i] & 0b10000000) == 0 {
            return Ok((res, i + 1));
        }
    }

    Err(())
}

pub async fn read_chunk(
    buf: Box<Vec<u8>>,
    length: i32,
) -> Result<(i32, Box<Vec<i8>>), ()> {

    // println!("c:{:?}", buf);

    let id = match read_varint(&buf) {
        Err(_) => return Err(()),
        Ok(v) => v,
    };

    let mut data = Box::new(Vec::with_capacity((length as usize) - id.1));

    // println!("{}", ((length as usize) - id.1));

    for b in buf[((length as usize) - id.1)..].iter() {
        data.push(*b as i8);
    }

    Ok((id.0, data))
}
