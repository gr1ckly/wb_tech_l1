pub fn change_bit(chislo: i64, i:usize, value: u8) -> i64{
    if i<=63 && i>=0 && (value ==0 || value ==1){
        let position = i / 4;
        let ost = i%4;
        let mut dvoich = chislo.to_le_bytes();
        let mut curr_byte = dvoich[position];
        let mut byte = Vec::new();
        for i in 0..8{
            byte.push(curr_byte % 2);
            curr_byte = curr_byte / 2;
        }
        byte[ost] = value;
        let mut new_byte = 0;
        for j in 0..8{
            new_byte += byte.pop().unwrap() * 2_u8.pow(7 - j);
        }
        dvoich[position] = new_byte;
        return i64::from_le_bytes(dvoich)
    }
    chislo
}