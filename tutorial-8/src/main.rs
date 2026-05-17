fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v2 = vec![2, 3, 4];
    v.push(2);
    v.push(2);
    v.push(2);
    // &v[i]    -> &T          : 범위 벗어나면 panic!
    // v.get(i) -> Option<&T>  : 범위 벗어나면 None (안전)
    // 둘 다 참조 반환 (깊은 복사 아님). 값 복사가 필요하면 .copied() / .cloned()
    // 성능: 둘 다 bounds check 수행, 차이는 실패 시 동작(panic vs Option)뿐
    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];

    let first = v[0];

    v.push(6);
    println!("The first element is: {first}");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // String의 `+`는 `impl Add<&str> for String` 한 가지만 존재:
    //   fn add(self, other: &str) -> String
    // → 왼쪽: String(소유권 이동), 오른쪽: &str(참조)
    // 따라서 `&s1 + &s2`나 `s1 + s2`는 컴파일 실패. 반드시 `s1 + &s2`.
    // (&s2는 deref 강제 변환으로 &String -> &str)
    //
    // 동작: s1의 힙 버퍼를 그대로 재사용(이동, 복사 없음)
    //       + s2의 바이트를 deep copy 해서 append (capacity 부족하면 재할당)
    // 결과 s3는 하나의 연속된 버퍼. s2 안의 참조를 들고 있는 게 아님.
    // → s1은 move되어 사용 불가, s2는 빌리기만 했으므로 그대로 사용 가능.
    //
    // 왜 이렇게 비대칭? 오른쪽도 String으로 받으면 매번 clone() 강제됨.
    // 왼쪽은 버퍼 재사용을 위해 이동, 오른쪽은 재사용 가능하도록 빌림 → 효율 + 유연성.
    let s3 = s1 + &s2;

    println!("{}", s3);
    // println!("{}", s1); // 컴파일 에러: s1은 + 에서 s3로 이동됨
    println!("{}", s2); // OK: &s2로 빌렸을 뿐이라 여전히 유효

    // 둘 다 살리고 싶으면 format! (내부적으로 양쪽 모두 빌림, 새 버퍼 할당)
    // let s3 = format!("{s1}{s2}");

    let hello = "Здравствуйте";
    // String/&str의 `[]`는 글자(char)가 아니라 **바이트** 단위 슬라이싱.
    // 키릴 문자는 UTF-8에서 글자당 2바이트라 [0..1]은 'З'의 중간 → 런타임 panic
    //   ("byte index 1 is not a char boundary")
    // Rust가 `hello[0]` 같은 정수 인덱싱을 막아둔 이유:
    // 글자마다 1~4바이트라 "n번째 글자"를 O(1)로 못 줌.
    let answer = &hello[0..1]; // ⚠️ 실행 시 panic. ASCII만 안전.

    // 첫/n번째 "글자"를 얻으려면 chars() 이터레이터 사용 (UTF-8 해석해줌)
    //   hello.chars().next()   → 첫 char (Option<char>)
    //   hello.chars().nth(n)   → n번째 char, O(n)
    // ※ 자모 분리(ㄱ+ㅏ)나 이모지 조합은 한 "글자"가 여러 char일 수 있음.
    //   그럴 땐 unicode-segmentation 크레이트의 .graphemes(true) 사용.
    let third = hello.chars().nth(2);
    // third는 Option<char>. Option은 Display(`{}`) 미구현 → `{:?}`(Debug)로 출력
    // 또는 unwrap() / unwrap_or('?') 등으로 char를 꺼낸 뒤 `{}` 사용.
    println!("{:?}", third); // Some('р')

    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // HashMap::get(&K) → Option<&V>  (HashMap이 값의 소유권을 그대로 쥐고 빌려줌)
    //   .copied()    : Option<&i32> → Option<i32>  (i32: Copy 이므로 무료 복사)
    //                  String/Vec 같은 비-Copy 타입은 .cloned() 사용 (할당 발생)
    //   .unwrap_or(0): Some(x) → x, None → 0  (키 없을 때 기본값 0점 처리)
    //
    // [Option 다루는 패턴 정리]
    //   x.unwrap()              : None이면 panic (확실히 안전할 때만)
    //   x.expect("msg")         : panic + 메시지 (디버깅용)
    //   x.unwrap_or(기본값)     : None이면 기본값
    //   x.unwrap_or_else(|| ..) : 기본값이 무거운 연산이면 lazy
    //   x.unwrap_or_default()   : T::default() 사용 (i32 → 0)
    //   x.map(|n| n*2)          : Some 안의 값만 변환
    //   x.and_then(|n| ..)      : Option 반환 함수 연쇄
    //   x?                      : 함수에서 None 즉시 전파
    //   if let Some(n) = x      : 한 케이스만 관심
    //   match x { Some/None }   : 양쪽 다 처리
    //   println!("{:?}", x)     : Option 구조 그대로 출력 (Some(10)/None)
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // ─────────────────────────────────────────────────────────────
    // [Rust의 안전성 보장 정리]
    //
    // 컴파일 타임에 박제되는 것 (소유권/메모리):
    //   - use-after-free, double-free, null 역참조, dangling pointer, data race
    //   → 카테고리 자체가 불가능. C/C++ 메모리 버그의 대부분이 사라짐.
    //
    // 동시성: data race는 막아주지만 모든 동시성 버그는 아님
    //   ✅ Data race        : 컴파일 에러 (Send/Sync + borrow checker)
    //   ❌ Deadlock         : 런타임에 멈춤 (로직 차원)
    //   ❌ Race condition   : 비즈니스 로직 차원
    //   ❌ 채널 무한 대기, async 태스크 leak 등
    //
    // Send  : 스레드 사이로 소유권 이동 가능 (Arc ✓, Rc ✗)
    // Sync  : 여러 스레드에서 동시 참조 가능 (Mutex ✓, RefCell ✗)
    // → 스레드 안전한 타입만 스레드에 보낼 수 있다는 규칙이 타입 시스템에 박혀있음.
    //
    // 이게 "fearless concurrency": 메모리 깨질까 무서워 안 해도 되고, 데드락만 신경.
    // ─────────────────────────────────────────────────────────────

    // ─────────────────────────────────────────────────────────────
    // [HashMap::entry — Entry API]
    //
    // entry(key) → Entry<K, V>  (enum: Occupied / Vacant — 해시 1회로 자리 확보)
    //   .or_insert(v)         → &mut V  (없으면 v 삽입, 있으면 그대로)
    //   .or_insert_with(|| ..)→ &mut V  (lazy 기본값 — 무거운 연산일 때)
    //   .or_default()         → &mut V  (Default::default())
    //   .and_modify(|v| ..)   → Entry   (있을 때만 수정, 체인 가능)
    //
    // 핵심 멘탈 모델: "값"이 아니라 "자리(슬롯)에 대한 핸들"을 받는다.
    // → upsert(있으면 update, 없으면 insert)의 Rust식 표현.
    //
    // 예) 단어 카운팅:
    //   for word in text.split_whitespace() {
    //       *count.entry(word).or_insert(0) += 1;
    //   }
    //
    // ⚠️ or_insert는 키가 이미 있으면 덮어쓰지 않음. 강제 교체는 insert 직접.
    //
    // [Rust enum = sum type (tagged union)]
    // Entry처럼 Rust enum은 각 variant가 데이터를 품음. C++의 정수 enum과 다름.
    // C++ 대응: std::variant<A,B,C> (C++17), Option ≈ std::optional, Result ≈ std::expected(C++23)
    // 차이: Rust는 언어 1급 + match exhaustiveness 강제 → variant 추가 시 누락된 분기를
    //       컴파일러가 다 찾아줌. C++ std::visit는 빠뜨려도 통과 → 런타임 버그.
    // ─────────────────────────────────────────────────────────────
    scores.entry(String::from("Yellow")).or_insert(50); // 이미 50 → 변화 없음
    scores.entry(String::from("Blue")).or_insert(50);   // 이미 10 → 그대로 10 (덮어쓰지 않음)
}
