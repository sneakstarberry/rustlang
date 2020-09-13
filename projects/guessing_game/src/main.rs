use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("숫자를 맞혀봅시다.");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("사용자가 맞혀야 할 숫자{}", secret_number);
    loop{
        println!("정답이라고 생각하는 숫자를 입력하세요.");
        //입력 받은 값을 저장할 공간 확보
        //let = 변수 선언 기본적으로 변수가 변화 할수 없다. amazing
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("입력한 값: {}", guess);



        match guess.cmp(&secret_number) {
            Ordering::Less => println!("입력한 숫자가 작습니다.!"),
            Ordering::Greater => println!("입력한 숫자가 큽니다.!"),
            Ordering::Equal => {
                println!("정답!");
                break;
            }
        }
    }
}