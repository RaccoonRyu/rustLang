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

    
}
