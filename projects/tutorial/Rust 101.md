# Rust 101

- `let`, 기본적으로 rust에서 변수는 값이 변하지 않는다.

```rust
let foo = 5; //불변한 값, 상수
let mut bar = 5; //가변 변수, 변할 수 있는 값
```
### 카고 명령어
---
- `cargo new`{프로젝트 이름} : 카고로 새로운 프로젝트를 시작할 때 기본 환경을 만들 수 있다.
- `cargo run`: 프로젝트 빌드와 동시에 실행할 수 있다.
- `cargo build`: 프로젝트를 빌드한다.
### 구조체

---

- 구조체 정의

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
```

- 구조체의 인스턴스 생성

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active:true,
    sign_in_count: 1,
}
// 가변 구조체의 필드 값 변경
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active:true,
    sign_in_count: 1,
}

user1.email = String::from("anotheremail@example.com")
```

- 같은 이름의 필드와 변수를 편리하게 활용하기

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
//필드와 변수의 이름 같을 때 둘 중 하나만 써도 같은 결과를 얻을 수 있다.
```

- 기존의 인스턴스로 부터 새 인스턴스 생성

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count
}
// or
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
}
// email과 username 필드에는 새로운 값을 대입하고 나머지 필드에는 user1 변수에 저장된 인스턴스의 필드값들을 대입하는 새 인스턴스 생성 코드
```

- 이름 없는 필드를 가진 튜플 구조체로 다른 타입 생성하기

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
// 위 둘은 서로 다른 타입이다. 서로 다른 튜플 구조체의 인스턴스이기 때문이다.
```

- 필드가 없는 유사 유닛 구조체(unit-like structs)

러스트에서는 필드가 하나도 없는 구조체를 선언할 수도 있다. 이런 구조체를 유사 유닛 구조체(unit-like structs)라고 한다. 제 10장에서 제대로 알아보자

``` rust
sturct User{
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    }
}
//이러면 &str에 의해 오류가 발생한다.
//String 같이 소유할 수 있는 타입을 사용해서 오류를 회피한다.
```



### 라이브러리

---

- `std::io`, 기본적으로 사용자가 입력한 값을 읽는 기능을 비롯해 여러 가지 유용한 기능을 제공한다.

```rust
use std::io
```

#### io

---

- `stdin`, io 관련 함수로 사용자 입력 값을 읽을 수 있도록한다.

```rust
use std::io;
fn main() {
    let mut guess = String::new();
    
	io::stdin().read_line(&mut guess)    
}

```

만약, 프로그램의 시작 부분에 `use std::io`를 작성하지 않는다면 이 코드는 `std::io::stdin`과 같이 작성 가능하다.

 `read_line`의 메서드 역할은 사용자가 입력한 값을 표준 입력으로 읽어 문자열에 저장한다. 그래서 문자열 인스턴스를 인수로 전달해 준다.

#### 크레이트 crate

---

크레이트는 rust개발자들이 제공하는 소스 파일의 집합이다.

크레이트는  Cargo.toml 파일을 수정해서 이용할 수 있다. 예시는 다음과 같다.

```toml
[dependencies]

rand="0.6.1"
```

이후 `cargo build`명령어를 입력하게 되면 크레이트 파일을 다운 받고 컴파일하게 된다.

 카고는 누구나 이코드를 빌드하더라도 매번 같은 결과 물을 재생산하도록한다. 예를 들어, 당신이 패키지의 중요한 버그를 수정했지만 이후 다음 버전이 출시된다면 어떻게 될까?

이 해답은 바로 `'Cargo.lock'`파일이다. 이 파일은 최초에 `cargo build`를 실행 할 때 발생하고 이 안에 패키지의 버전을 기록한다. 이를 통해 우린 고정된 버전의 패키지를 이용하여 언제나 같은 생산성을 유지할 수 있다.

그렇다면 새 버전으로 업그레이드는 어떻게 할까? `cargo update`라는 명령어를 입력하면 0.6.1 이라면 0.6.1보다는 높고 0.7.0 버전 보다는 낮은 버전의 크레이트를 생성한다. 만약 0.6.x 가 아닌 0.7.0 버전을 이용하고 싶다면 다음과 같다.

```toml
[dependencies]
rand = "0.7.0"
```



#### 자리지정자

---

- println!("{}", {value}), {} 사이에 value 값이 입력된다.

```rust
println!("입력한 값: {}", guess) // {}안에 guess 값이 들어간다.
```

만일 여러 개의 값을 출력하고자 한다면 다음과 같다.

```rust
let x = 5;
let y = 10;

println!("x ={}, y = {}", x,y)
```



# 타입

#### 정적메서드 - 타입에 구현된 함수 지칭

---

- `new`, `new`함수는 어떤 타입의 새로운 값을 생성하는 함수에 일반적으로 부여하는 이름이다.

```rust
fn main() {
    let mut guess = String::new(); //새로운 빈 문자열을 생성한다.
}
```

### 참조

---

- `&`이 기호는 이 인수가 '참조'(포인터) 이라는 점을 지시한다.

### Result 타입

---

Result 타입은 '열거자(enumerations)'로, 줄여서 'enums'로 표기하기도 한다. 열거자는 미리 정의된 범위의 값을 갖는 타입이며 이 값들은 열거자의 '열것값(variants)'이라고 부른다.  

Result 열거자의 경유, 열것값은 ok와 Err가 있다. Ok 열것값은 작업이 성공적으로 완료되었다는 것을 의미하며, Ok 열것값에는 성공적으로 생성된 값을 보관한다. 반면, Err 열것값은 작업이 실패했음을 의미하며, 해당 작업이 실패한 원인에 대한 정보를 보관한다.

**Result 타입의 목적**은 에러 처리를 위한 정보를 인코딩하기 위한 것이다. 다른 타입의 값과 마찬가지로 Result 타입의 값은 각자 자신에게 정의된 메서드를 포함하고 있다.

##### 오류처리하기

- `.expect`,  io::result 인스턴스의 값이 Err라면 expect 메서드는 프로그램의 실행을 종료하고 expect 메서드에 인수로 전달한 메시지를 표시한다.