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
            <h1 class="title">함수 사용하기</h1>
            <p>
                이번 강의에서는 러스트에서 함수를 사용하는 방법에 대해 알아보겠습니다.
            </p>
        </article>
        <hr>
        <article>
            <h1 class="title">함수 선언 및 호출</h1>
            <p>다른 언어들처럼 러스트에서도 반복 사용되는 구문은 함수로 만들어 재사용할 수 있습니다.</p>
            <p>
                러스트에서 함수는 <span class="highlight">fn 예약어를 사용하여 선언</span>합니다.
                <a href="#ex3-1" class="example">[ex3-1]</a> 7행은 ex_function 이라는 이름의 함수를 선언하고 
                함수 내용으로는 "This is an example function." 문자열을 출력하고 있습니다.
                fn [함수명] () &#123 [함수내용] &#125 형식으로 선언합니다.
            </p>
            <p>
                선언된 변수를 호출하기 위해서는 [함수이름](); 형식을 사용합니다. <a href="#ex3-1" class="example">[ex3-1]</a>
                3행은 7행에서 선언된 ex_example 함수를 main 함수에서 호출하고 있습니다.
            </p>
			<Codeblock id = {"ex3-1"} range = {"1, 9"} path = {"/rust/example/examples/ex3-1.rs"}/>
        </article>
        <hr>
        <article>
            <h1 class="title">함수에 인자 넘겨주기</h1>
            <p>함수에 인자를 전달하는 방법에 대해 알아보겠습니다.</p>
            <p>
                <span class="highlight">호출되는 함수내에서는 호출하는 함수에서 선언된 변수를 알 수 없습니다.</span>
                따라서 호출되는 함수에서 호출하는 함수에서 선언된 변수를 사용하기 위해서는 해당 변수를 인자로 넘겨줘야 합니다.
                인자를 입력받는 함수는 fn [함수명] ([인자명]: [자료형], ...) &#123 [함수내용] &#125 형식으로 선언합니다.
            </p>
            <p>
                <a href="#ex3-2" class="example">[ex3-2]</a>은 main 함수안에 선언된 변수 a에 1을 더하는 기능을 하는 plus_one함수를 호출하고 있습니다.
                (호출하는 함수: main, 호출되는 함수: plus_one) 9행에서 plus_one함수를 선언할 때 인자로 넘겨받을 변수 value를 자료형과 함께 나타내고 있습니다.
                이후 value에 1을 더한 후 println! 매크를 이용해 출력을 합니다.
                5행에서는 plus_one 함수를 호출할 때 <span class="highlight">변수 a를 인자로 보내 plus_one함수에서 사용할 수 있게 하고 있습니다.</span>
            </p>
			<Codeblock id = {"ex3-2"} range = {"1, 11"} path = {"/rust/example/examples/ex3-2.rs"}/>
        </article>
        <hr>
        <article>
            <h1 class="title">반환값이 있는 함수</h1>
            <p>반환값이 있는 함수에 대해 알아보겠습니다.</p>
            <p>
                호출되면 <span class="highlight">결과값을 반환하는 함수</span>를 선언할 수 있습니다.
                fn [함수명]([인자명]: [자료형], ...) -> [(반환)자료형] &#123 [함수내용] &#125 형식으로 값을 반환하는 함수를 선언할 수 있습니다.
                <a href="#ex3-3" class="example">[ex3-3]</a> 9행은 반환값이 있는 plus_one 함수를 선언하고 있습니다.
                컴파일러에게 반환되는 자료형이 무엇인지 알려주기 위해 -> [자료형] 형식을 사용하고 있습니다. 그리고 13행에서 반환되는 값을 적어줍니다.
                이 때 주의해야 할 사항으로는 <span class="highlight">반환되는 값을 나타낼때는 세미콜론이 없어야 한다</span>는 것입니다.
                다른 방법으로 12행처럼 return 함수를 사용할수도 있습니다.
            </p>
            <p>
                <a href="#ex3-3" class="example">[ex3-3]</a> 4행에서는 plus_one 함수를 호출하여 반환된 값을 ret 변수에 할당하는 예를 보여주고 있습니다.
                plus_one 함수에 변수 a를 인자로 보내 호출하면 1이 반환되어 변수 ret에 할당됩니다. 이후 6행에서 변수 ret를 출력하면 "returned value: 1"이 나타납니다.
            </p>
			<Codeblock id = {"ex3-3"} range = {"1, 14"} path = {"/rust/example/examples/ex3-3.rs"}/>
        </article>
        <hr>
        <article>
            <h1 class="title">요약</h1>
            <ul class="summary">
                <li>반복 사용되는 구문은 함수로 만들어 재사용할 수 있다.</li>
                <li>함수는 fn 예약어를 사용하여 선언한다.</li>
                <li>함수에 인자를 넘겨주기 위해서는 함수 선언부에 인자 이름과 자료형을 표기한다.</li>
                <li>값을 반환하는 함수를 선언하기 위해서는 선언부에 반환되는 자료형을 표기하고, <span class="highlight">함수내용 마지막 줄에는 반환되는 값을 세미콜론 없이 표기한다.</span></li>
            </ul>
        </article>
    </section>
</main>

<style>
	@import '/static/css/lecture.css';
</style>