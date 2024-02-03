fn main () {
    println!("test");
    let result = ex1089([1,2,3,2,9]);
    println!("{result}");
}

fn ex1089(music_data:[i32;5])->i32{

    let first_value = music_data[0];
    let last_value = music_data[music_data.len()-1];
    let mut qt_extreme_values = 0;

    println!("first:{}, last:{}",first_value,last_value);
    for index in 0..music_data.len() {
        if index==0 {
            //use last value
            if is_extreme_value(last_value,music_data[index],music_data[index+1]){
                qt_extreme_values+=1;
            }
        }
        else if index==(music_data.len()-1) {
            //use first value
            if is_extreme_value(music_data[index-1],music_data[index],first_value) {
                qt_extreme_values+=1;
            }
        }
        else{
            //normal operation
            if is_extreme_value(music_data[index-1],music_data[index],music_data[index+1]) {
                qt_extreme_values+=1;
            }
        }
    }
    qt_extreme_values
}

fn is_extreme_value(vb:i32,vm:i32,va:i32)->bool{
    let sum_before = vb-vm;
    let sum_after = va-vm;
    if(sum_before>=0 && sum_after>=0) || (sum_before<=0 && sum_after<=0){
        return true;
    }
    false
}