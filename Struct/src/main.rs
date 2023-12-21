fn main() {
    // 구조체
    // 튜플과 유사
    // 튜플과 마찬가지로 구조체는 각기 다른 타입의 값으로 구성됨
    // 하지만 튜플과는 달리 각 데이터에 별개의 이름을 부여하여 값의 의미를 더 분명히 표현할 수 있다.
    // 또한 각 데이터에 이름이 있으므로 튜플보다 유연하다.
    // -> 참조할 데이터를 가리키거나 인스턴스의 값을 읽을 때 데이터의 순서에 의존할 필요가 없기 때문

    struct User { // 구조체 정의 : struct 키워드 + 구조체 이름 {}
        
        // 중괄호 안에는 구조체가 저장할 데이터 타입 + 이름을 나열 (필드)
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    };

    let user1 = User { // 구조체 정의 후 실 사용을 위한 User 구조체의 인스턴스 생성

        // 필드에 저장할 값을 명시하여 구조체의 인스턴스 생성
        // 필드의 나열 순서는 구조체에 정의한 순서와 반드시 같을 필요 없음
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1, // 키(필드명):값(필드에 저장할 데이터) 의 쌍

    };

    let mut user2 = User { // 구조체는 몇몇 필드만을 가변 데이터로 표시하는 것을 지원하지 않는다. (인스턴스 자체가 가변이다.)

        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,

    };

    user2.email = String::from("anotheremail@example.com");

    // 또한 구조체의 새로운 인스턴스도 함수를 이용해 생성할 수 있다.
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    } // 이 때 함수의 마지막 표현식은 묵시적으로 새 인스턴스를 리턴해야 함.

    // 기존의 인스턴스에서 몇 가지 필드 값만 수정하여 새 구조체 인스턴스 생성 (구조체 갱신 문법)
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user2.active,
        sign_in_count: user2.sign_in_count,
    };

    // 구조체 갱신 문법 사용시 더 적은 코드로 같은 결과를 얻을 수 있다.
    // ..문법 : 명시적으로 나열하지 않은 나머지 필드에 대해서는 기존의 인스턴스의 필드와 같은 값을 사용하라는 의미
    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user3
    };

    // 튜플과 유사하게 생긴 구조체 선언 - 튜플 구조체
    // 튜플 구조체는 구조체에 이름을 부여하지만, 필드에는 이름을 부여하지 않고 타입만 지정한다.
    // 튜플 자체에만 이름을 부여해 여타 튜플들과는 다른 타입으로 구분하고자 할 때 유용하다.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32); // 튜플 구조체 정의

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0); // 튜플 구조체 인스턴스 생성. black과 origin은 서로 다른 튜플 구조체의 인스턴스이기 때문에 서로 다른 타입이다.

    // 유사 유닛 구조체
    // 필드가 하나도 없는 구조체
    // 어떤 타입의 트레이트(10장에서 다룸)를 구현해야하지만 타입에 저장할 데이터가 없을 때 유용히 활용 가능

    // 구조체 사용 예제
    // 사각형 면적 구하는 프로그램 작성하기

    let width1 = 30;
    let height1 = 50;

    println! (
        "사각형의 면적: {} 제곱 픽셀",
        area(width1, heigth1)
    );

    fn area(width: u32, height: u32) -> u32 {
        width * height
    }

    // 위의 소스를 튜플 이용하여 리팩토링
    let rect1 = (30, 50);

    println!(
        "사각형의 면적: {} 제곱 픽셀",
        area2(rect1)
    );

    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1 // 이전과 비교시 하나의 튜플 인수만 전달하면 된다
        // 하지만 튜플은 요소에 이름을 부여하지 않으므로, 튜플의 요소에 인덱스로 접근한다.
        // 따라서 각 요소의 의미가 명확하지 않다.
        // 최종적으로 데이터의 의미를 코드에 반영하지 못하면,
        // 해당 코드의 의미를 잊어버리거나 오사용해서 에러를 발생시킬 가능성이 더 커진다.
    }

    struct rect2 { // 구조체 선언
        width: u32,
        heigth: u32, // u32 타입 필드 선언
    }

    let useRect2 = rect2 { width: 30, height: 50 };

    println!("사각형의 면적 : {} 제곱 픽셀", area3(&rect2));

    fn area3(rectangle : &rect2) -> u32 { // rect2라는 매개변수 사용 -> rect2 구조체의 불변 인스턴스에 대한 대여 타입
        rect2.width * rect2.height
    } // area 함수는 rect2 구조체 인스턴스의 width, height 필드를 이용해 계산을 수행한다.
    // 즉, 해당 함수는 width와 height 필드 값을 이용해 주어진 rect2 인스턴스의 면적을 계산한다는 점을 명확히 표현함.
    // 또한 구조체를 이루는 변수가 서로 관련이 있다는 점도 충분히 표현하며, 변수에 의미있는 이름 또한 부여할 수 있다. (코드의 명확성이 좋아졌다.)

    // 러스트는 디버깅 정보를 출력하는 기능을 제공한다.
    // 하지만 구조체는 이 기능을 명시적으로 구현해줘야 한다. (기본적으로 구조체의 값을 출력하는 코드가 정상 작동하지 않는다.)
    // 따라서 #[derive(Debug)] 애노테이션을 구조체 정의에 추가하여 구조체의 값을 확인할 수 있다.
    #[derive(Debug)]
    struct rect3 {
        width: u32,
        height: u32,
    }

    let useRect3 = rect3 { width: 30, height: 50};
    println!("rect3 : {:?}", useRect3); // println! 매크로 형식 문자열 안에 {:?}을 이용해서 구조체 인스턴스의 모든 필드값을 확인할 수 있다.

    // 메서드 문법
    // method는 함수와 유사하다.
    // 함수와 마찬가지로 fn 키워드를 이용해 정의하며, 이름/매개변수/리턴 타입을 정의할 수 있다.
    // 또한 함수와 마찬가지로 호출 시 실행할 일련의 코드를 정의하고 있다.

    // 하지만 메서드는 함수와 달리 구조체의 컨텍스트(context) 안에 정의하며,
    // 첫 번쨰 매개변수는 항상 메서드를 호출할 구조체의 인스턴스를 표현하는 self여야 한다.

    // 메서드 정의
    #[derive(Debug)]
    struct rect4 {
        width: u32,
        height: u32,
    }

    impl rect4 { // 구조체 타입의 컨텍스트 안에 함수를 정의하기위한 impl 블록 사용
        fn area4(&self) -> u32 { // 이후 함수를 impl 중괄호 안으로 이동하고
            self.width * self.height // 첫 번째 매개변수명과 함수 본문 안에서의 참조를 모두 self로 바꾼다 -> 기존 방식
        }
    }

    let useRect4 = rect4 { width : 30, height : 50 };

    println!("사각형의 면적 : {} 제곱픽셀", useRect4.area4()); // 기존 방식과 달리 area4 메서드를 rect4 인스턴스상 호출할 수 있다.

    
}
