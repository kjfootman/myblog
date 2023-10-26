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
            <h1 class="title">변수의 종류</h1>
            <p>
                러스트에서는 여러가지 자료형을 제공하고 있지만 자주 사용되는 자료형에 대해 알아보도록 하겠습니다.<br>
                여러가지 자료형이 나열되겠지만, 자주 사용하는 것들만 우선적으로 익히고 러스트로 개발을 하면서 필요한 내용을 더 익히는것을 추천드립니다
            </p>
            <hr>
        </article>
        <article>
            <h1 class="title">정수형</h1>
            <p>정수를 저장할 수 있는 변수들에 대해 알아보겠습니다.</p>
            <p>
                정수형에는 총 12가지 자료형이 있습니다. 자료형에 따라 표현할 수 있는 최대 자리수가 달라지며, 양수만 할당할 수 있는 자료형도 있습니다.
                아래 표와 같이 <span class="highlight">부호를 표시할 수 있는 정수형</span>은 i로 시작되고 
                <span class="highlight"> 표시할 수 없는 정수형</span>은 u로 시작됩니다.
                부호가 없는 자료형(u로 시작하는 자료형)은 주로 배열의 인덱스를 나타내는데 사용됩니다.
            </p>
            <p>
                i나 u뒤에 붙은 숫자는 bit단위 크기를 타나냅니다. 숫자가 클수록 더 긴 숫자를 저장할 수 있습니다.
            </p>
            <table style="width: 300px; text-align: center;">
                <thead>
                    <tr>
                        <th>Length</th>
                        <th>Signed</th>
                        <th>Unsigned</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>8-bit</td>
                        <td>i8</td>
                        <td>u8</td>
                    </tr>
                    <tr>
                        <td>16-bit</td>
                        <td>i16</td>
                        <td>u16</td>
                    </tr>
                    <tr>
                        <td>32-bit</td>
                        <td>i32</td>
                        <td>u32</td>
                    </tr>
                    <tr>
                        <td>64-bit</td>
                        <td>i64</td>
                        <td>u64</td>
                    </tr>
                    <tr>
                        <td>128-bit</td>
                        <td>i128</td>
                        <td>u128</td>
                    </tr>
                    <tr>
                        <td>arch</td>
                        <td>isize</td>
                        <td>usize</td>
                    </tr>
                </tbody>
                <caption>정수형 변수의 종류</caption>
            </table>
            <p>
                <a href="#ex2-1" class="example">[ex2-1]</a>에서는 몇 가지 정수형 변수를 선언하고 더하기 연산을 수행했습니다.
                <span class="highlight">러스트에서는 자료형이 같은 변수끼리만 연산이 가능</span>합니다.
                4~5행에서 각각 i32형 변수 a와 b를 선언하고 6과 4를 할당했습니다. 이후 8행에서 더하기 연산을 수행하면서 변수 c를 선언했습니다.
                만약 7행처럼 변수 b를 i64형으로 선언하고 변수 a와 더하기 연산을 시도하면 자료형이 다르기 때문에 에러가 발생합니다.
            </p>
			<Codeblock id = {"ex2-1"} range = {"3, 19"} path = {"/rust/example/examples/ex2-1.rs"}/>
            <hr>
        </article>
        <article>
            <h1 class="title">실수형</h1>
            <p>
                실수형에는 f32와 f64 두 종류가 있습니다. 
                정수형과 마찬가지로 f뒤에 나오는 숫자는 변수의 크기이며 숫자가 클수록 더 긴 자리수를 나타낼 수 있습니다.<br>
            </p>
            <table style="width: 200px; text-align: center;">
                <thead>
                    <tr>
                        <th>Length</th>
                        <th>Data type</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>32-bit</td>
                        <td>f32</td>
                    </tr>
                    <tr>
                        <td>64-bit</td>
                        <td>f64</td>
                    </tr>
                </tbody>
                <caption>실수형 변수의 종류</caption>
            </table>
            <p>
                <a href="#ex2-2" class="example">[ex2-2]</a>에서는 실수형 변수를 선언하고 더하기 연산을 수행했습니다.
                4~5행에서 각각 f32형 변수 a와 b를 선언하고 6.0과 4.0을 할당합니다.
                값을 할당할 때 <span class="highlight">실수형임을 나타내기위해 값 뒤에 .0을 붙이거나 자료형을 붙여줍니다.</span><br>
                만약 7행처럼 정수형 값을 f32형 변수에 할당할 경우 에러가 발생합니다.
            </p>
            <p>
                정수형 예제에서와 마찬가지로 자료형이 같을때만 연산을 수행할 수 있습니다.
            </p>
			<Codeblock id = {"ex2-2"} range = {"3, 21"} path = {"/rust/example/examples/ex2-2.rs"}/>
            <hr>
        </article>
        <article>
            <h1 class="title">논리형</h1>
            <p>
                논리형 변수에는 <span class="highlight">참(true)/거짓(false) 이진 정보만 저장</span>할 수 있습니다.
            </p>
            <p>
                <a href="#ex2-3" class="example">[ex2-3]</a> 4~5행은 논리형 변수 a와 b를 선언하고 각각 true, false 값을 할당하였습니다.
                이후 7~9행에서 각각 and, or, xor 연산을 수행하고 결과를 출력했습니다.
            </p>
			<Codeblock id = {"ex2-3"} range = {"3, 9"} path = {"/rust/example/examples/ex2-3.rs"}/>
            <hr>
        </article>
        <article>
            <h1 class="title">튜플형</h1>
            <p>
                튜플형은 변수들의 묶음으로 생각할 수 있습니다.
            </p>
            <p>
                let [변수명]: (자료형, 자료형, ...) = (변수값, 변수값, ...); 형식으로 선언할 수 있습니다.
                <a href="#ex2-4" class="example">[ex2-4]</a> 4행은 튜플변수를 선언하고 값을 할당하고 있습니다. 
                괄호안의 자료형 개수와 할당값 개수가 같아야 에러가 발생하지 않습니다.
                5행은 자료형을 생략하여 변수를 생성하고 값을 할당하고 있습니다. 
            </p>
            <p>
                튜플 안 <span class="highlight">각각의 값(요소)은 [변수명].[[인덱스]] 형식으로 접근</span>할 수 있습니다. 
                인덱스는 튜플 안에서 몇 번째 위치인지 나다내는 숫자이며 0부터 시작합니다.
                8~11행은 튜플변수 t와 u의 각 성분들끼리 곱해서 값을 출력하는 예제입니다.
            </p>
			<Codeblock id = {"ex2-4"} range = {"3, 11"} path = {"/rust/example/examples/ex2-4.rs"}/>
            <hr>
        </article>
        <article>
            <h1 class="title">배열형</h1>
            <p>
                배열형 변수는 자료형이 같은 변수들의 묶음으로 생각할 수 있습니다.
            </p>
            <p>
                <a href="#ex2-5" class="example">[ex2-5]</a>는 배열형 변수를 선언하고 각 성분들에 접근하는 방법을 보여주고 있습니다. 
                배열형 변수 let [변수명]: [자료형, 자료형, ...] = [변수값, 변수값, ...] 형식으로 선언합니다.
                4행은 f64형 값 5개를 포함하는 변수 a를 언하고 1.0부터 5.0까지 값을 할당하였습니다.
                5행은 자료형을 생략하고 변수 b를 선언하고 5.0부터 1.0까지 5개 값을 할당하였습니다.
            </p>
            <p>
                배열에 동일한 값을 여러개 할당하고 싶을 경우는 let [변수명] = [변수값; 개수]; 형식을 사용합니다
                6행은 배열형 변수 c에 2.0을 5개 할당하고 있습니다. 따라서 변수 c에는 [2.0, 2.0, 2.0, 2.0, 2.0]이 할당됩니다.
            </p>
            <p>
                배열의 <span class="highlight">각 요소에 접근하기 위해서는 [변수명][[인덱스]]형식을 사용</span>합니다. 
                예를 들어 변수 c의 첫 번째 요소는 c[0]로 접근할 수 있습니다.
                8~10행은 변수 a, b, c의 각 요소들로 간단한 연산결과를 출력하고 있습니다.
            </p>
			<Codeblock id = {"ex2-5"} range = {"3, 10"} path = {"/rust/example/examples/ex2-5.rs"}/>
            <hr>
        </article>
        <article>
            <h1 class="title">요약</h1>
            <ul class="summary">
                <li>정수형 변수에는 부호까지를 저장할 수 있는 변수와 양수만 저장할 수 있는 변수가 있다.</li>
                <li>i로 시작하는 변수는 음수값/양수값 모두 저장할 수 있으며, u로 시작하는 변수에는 양수값만 저장할 수 있다.</li>
                <li>실수형 변수에는 f32와 f64가 있다.</li>
                <li>실수형 변수에 값을 할당하기 위해서는 숫자 뒤에 소수점(.0)을 붙이거나 자료형(f32/f64)을 붙여준다.</li>
                <li>논리형 변수에는 bool이 있으며, and(&), or(|), xor(^) 연산이 가능하다</li>
                <li>튜플은 서로 다른 자료형들의 묶음으로 생각할 수 있으며, 각각의 요소는 [변수명].[[인덱스]] (t.0) 형식으로 접근 가능하다.</li>
                <li>배열은 동일한 자료형들의 묶음으로 생각할 수 있으며, 각각의 요소는 [변수명][[인덱스]] (a[0]) 형식으로 접근 가능하다.</li>
                <li><span class="highlight">자료형이 같은 변수들끼리만 연산이 가능하다.</span></li>
            </ul>
        </article>
    </section>
</main>


<style>
	@import '/static/css/lecture.css';
</style>