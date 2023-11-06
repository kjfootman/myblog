<script>
    import 'prismjs'
    import 'prismjs/components/prism-rust'
    import 'prismjs/plugins/file-highlight/prism-file-highlight'
    import 'prismjs/plugins/line-numbers/prism-line-numbers'
    import 'prismjs/plugins/line-highlight/prism-line-highlight'

    import Codeblock from '../../../codeblock.svelte'
</script>

<main>
    <section>
        <article>
            <h1 class="title">흐름 제어하기</h1>
            <p>이번 강의에서는 논리적 흐름을 제어하는 if, for, while 구문에 대해 알아보도록 하겠습니다.</p>
        </article>
        <article>
            <h1 class="title">if 구문 사용하기</h1>
            <p>if 문을 사용하면 조건에 따라 다른 명령을 실행할 수 있습니다.</p>
            <p>
                if 문은 if [조건절] &#123 [참일 경우] &#125 else &#123 [거짓일 경우] &#125 형식으로 사용합니다. 
                <a href="#ex4-1" class="example">[ex4-1]</a>은 if 문을 사용하는 몇 가지 예제를 보여주고 있습니다.
            </p>
            <p>
                7-11행은 변수 a가 홀수인지 짝수인지 출력하는 코드입니다. 
                우선, 7행 소괄호 안은 변수 a와 정수 2에 %연산을 수행하고 있습니다. 
                %연산은 a를 2로 나눈 나머지를 반환합니다. a에 4가 할당돼있기 때문에 a % 2 연산을 수행하면 0이 반환됩니다. 
                이후 == 논리연산자를 사용하여 소괄호안 나머지가 0인지 확인하여 0이면 "a는 짝수입니다"를 출력합니다. 
                만약 a에 5를 할당하면 조건절 (a % 2) == 0이 거짓이 되어 "a는 홀수입니다"가 출력됩니다.
            </p>
                <Codeblock id = {"ex4-1"} range = {"2, 11"} path = {"/rust/example/examples/ex4-1.rs"}/>
            <p>
                14~18행은 변수 a와 b가 같은 값인지 비교하는 예제입니다. 
                if 뒤 조건절 a == b가 참인지 확인하여 참이면 "a와 b는 같은 값입니다."를 출력하고, 거짓이면 "a와 b는 다른 값입니다"를 출력합니다.
            </p>
                <Codeblock id = {"ex4-1"} range = {"13, 18"} path = {"/rust/example/examples/ex4-1.rs"}/>
            <p>
                21-31행은 변수 a, b, c를 비교하는 예제입니다. 컴파일러는 우선 if 뒤 조건절 b &#60 a부터 확인합니다.
                조건절이 참이면 "b는 a보다 작은 수 입니다."를 출력합니다. 만약 거짓이면 컴파일러는 else if 뒤 조건절을 확인합니다.
                else if 뒤 조건절이 참이면 "b는 c보다 작은 수 입니다."를 출력합니다.
                만약 else if 뒤 조건절도 거짓일 경우는 "b는 a보다 크거나 같고 c보다 작거나 같습니다."가 출력됩니다.
                이처럼 else if 문은 순차적으로 조건절을 판별하고 결과에 따라 다른 명령을 실행할 수 있습니다.
                또한 26~28행처럼 else if 문은 경우에 따라 여러번 나올수도 있습니다.
            </p>
                <Codeblock id = {"ex4-1"} range = {"20, 31"} path = {"/rust/example/examples/ex4-1.rs"}/>
            <p>
                34-38행은 변수 d에 조건에 따라 다른 값을 할당하는 예제입니다. 34행에서 = 뒤에 if 문이 있는걸 볼수 있습니다.
                조건절 c % 2 == 0 이 참이면 변수 d에 0.0을 할당하고 거짓이면 1.0을 할당합니다. 
                "반환값이 있는 함수"에서와 마찬가지로 35,37행에서는 값을 반환하기 위해 세미콜론을 붙이지 않습니다. 
                주할 점은 35행 또는 37행에서 반환된 값이 변수 d에 할당되기 때문에 35행과 37행은 반드시 자료형이 같아야 합니다.
            </p>
            <Codeblock id = {"ex4-1"} range = {"33, 38"} path = {"/rust/example/examples/ex4-1.rs"}/>
            <details>
                <summary class="codesummary">
                    <span>ex4-1 전체코드 </span>
                </summary>
                <Codeblock id = {"ex4-1"} range = {"1, 41"} path = {"/rust/example/examples/ex4-1.rs"}/>
            </details>
        </article>
        <article>
            <h1 class="title">for 구문 사용하기</h1>
            <p>
                for를 사용하면 반복되는 작업을 쉽게 수행할 수 있습니다. 
                <a href="#ex4-2" class="example">[ex4-2]</a>는 for 구문을 사용하는 몇가지 예제입니다. 
            </p>
            <p>
                2-10행은 배열 arr에 저장된 각 성분을 출력하는 예제입니다. for문을 사용하지 않는다면<br> 
                <span style="display: block; text-align: center; margin: 20px;">
                    printlnt!("&#123 &#125", arr[0]);<br>
                    printlnt!("&#123 &#125", arr[1]);<br>
                    .<br>
                    .<br>
                    .<br>
                    printlnt!("&#123 &#125", arr[9]);<br>
                </span>
                와같이 각 성분들을 출력해야 합니다.
                만약 arr 성분의 개수가 100개라면 위와같이 println!을 100번이나 써줘야하기 때문에 코드가 장황해집니다.
                for문을 사용하면 7-9행 처럼 단 몇 3줄로 arr의 모든 성분을 출력할 수 있습니다. 
                <!-- 7행은 for [요소] in [배열] &#123 [명령문] &#125 형식을 나타내고 있습니다. 
                흐름을 따라가 보면, 7행에서 i에 arr의 첫번째 요소인 1이 할당됩니다. 이후 8행에서 i -->
            </p>
            <p>
                7행은 for [요소] in [시작]..[끝] 형태로 구성되 있는것을 볼 수 있습니다. 
                ..는 [시작]부터 [끝]-1 범위를 반환합니다. 주의할 점으로는 [끝]값은 범위에 포함되지 않습니다.
                7행에서 length()함수는 배열 arr의 요소 개수(길이)를 반환합니다. 
                따라서 arr에는 1부터 10까지 총 10개의 요소가 저장돼있기 때문에 10이 반환됩니다.
                즉, <span class="highlight">7행의 0..arr.length()은 0부터 9까지의 범위 (0..9)</span>를 반환합니다.<br>
                &nbsp흐름을 따라가보면, 7행에서 i에는 0..arr.length()의 첫 번째 값인 0이 할당됩니다. 
                이후 8행에서 i를 출력하고 다음 행인 9행으로 넘어가지 않고 7행으로 넘어갑니다.
                이 때 i에는 0..arr.length()의 두번째 값인 1이 할당되고 8행에서 i를 출력하는 과정을 반복합니다.
                7행에서 i가 0..arr.length()의 마지막 값인 9가 할당될 때까지 위 과정이 반복됩니다. 
                따라서 8행에서는 arr[9]까지 출력이 되면서 
            </p>
            <Codeblock id = {"ex4-2"} range = {"2, 10"} path = {"/rust/example/examples/ex4-2.rs"}/>
            <details>
                <summary class="codesummary">
                    <span>ex4-2 전체코드 </span>
                </summary>
                <Codeblock id = {"ex4-2"} range = {"1, 49"} path = {"/rust/example/examples/ex4-2.rs"}/>
            </details>
            <!-- <Codeblock id = {"ex4-2"} range = {"1, 49"} path = {"/rust/example/examples/ex4-2.rs"}/> -->
        </article>
    </section>
</main>

<style>
	@import '/static/css/lecture.css';
</style>