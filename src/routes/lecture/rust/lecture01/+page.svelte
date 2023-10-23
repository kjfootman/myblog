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
            <h1>변수 선언과 출력하기</h1>
			<p>
				이번 강의에서는 러스트 언어에서 사용되는 대표적인 자료형을 선언하고 출력해 보도록 하겠습니다.<br>
			</p>
        </article>
		<hr>
        <article>
            <h1>변수 선언하기</h1>
			<p>
				러스트에서는 let 예약어를 사용하여 변수를 선언합니다.<br>
				아래 <a href="#ex1-1">[ex1-1]</a>의 4행은 a라는 이름의 i32 정수형 변수를 선언하고 1을 할당하고 있습니다.<br>
				let [변수명]: [자료형] = [변수 값]<mark>;</mark> 형식으로 변수를 선언하고 있습니다.<br>
				주의할 점으로는 C언어나 Java처럼 한 줄의 명령이 끝나면 세미콜론(;)으로 닫아줘야 합니다.<br>
			</p>
			<p>
				러스트에서는 자료형을 명시적으로 적지 않아도 컴파일러가 자료형을 추정합니다.<br>
				<a href="#ex1-1">[ex1-1]</a> 5행은 b라는 변수를 선언하고 2를 할당하고 있습니다.<br>
				자료형을 적지 않았지만 컴파일러에서 i32 정수형으로 처리 합니다.<br>
			</p>
			<p>
				<a href="#ex1-1">[ex1-1]</a> 6행에서는 앞서 선언한 변수 a와 변수 b로 더하기 연산을 수행하고 결과값을 변수 c에 할당하면서 선언하였습니다.<br>
				변수 c도 자료형을 명시하지 않았지만 컴파일러에서 i32 정수형으로 처리합니다. (i32형 변수끼리 더하면 결과값으로 i32형이 반환 됩니다.)<br>
			</p>
			<p>
				여기까지 i32 정수형 변수를 선언하고 값을 할당하고 더하기 연산을 진행했습니다.<br>
			</p>
			<Codeblock id = {"ex1-1"} range = {"3, 11"} path = {"/rust/lecture01/example/examples/ex1-1.rs"}/>
		</article>
		<hr>
		<article>
            <h1>immutable 변수와 mutable 변수</h1>
			<p>
				앞서 설명드리지 않았지만 러스트에서 변수는 크게 immutable 변수와 mutable 변수로 구분할 수 있습니다.<br>
				기본적으로 변수는 선언 후 새로운 값을 할당할 수 없는데, 이는 사용자가 실수로 엉뚱한 값을 덮어 쓰는것을 방지하기 위함입니다. (immutable 변수로 선언)<br>
				이미 선언된 변수에 새로운 값을 덮어써야 하는 경우에는 해당 변수가 덮어쓰기를 허용하도록 선언돼야 합니다. (mutable 변수로 선언)<br>
				선언된 변수에 새로운 값을 덮어쓸 수 있는지 여부에 따라 mutable 변수와 immutable 변수로 구분할 수 있습니다.<br>
				아래 표에서 immutable 변수와 mutable 변수를 비교하였습니다.
			</p>
			<table border="1px solid black" style="border-collapse: collapse; margin: 0 auto; width: 80%;">
				<thead>
					<tr>
						<th></th>
						<th>immutable 변수</th>
						<th>mutable 변수</th>
					</tr>
				</thead>
				<tbody>
					<tr>
						<td>값 할당</td>
						<td>선언 후 값 할당 불가능</td>
						<td>선언 후 값 할당 가능</td>
					</tr>
					<tr>
						<td>용도</td>
						<td>읽기 전용으로 사용할 경우</td>
						<td>매번 변경되는 값을 저장하는 경우</td>
					</tr>
					<tr>
						<td>선언방법</td>
						<td>기본적으로 immutable 선언됨</td>
						<td>변수명 앞에 mut 키워드를 입력하여 선언</td>
					</tr>
				</tbody>
				<caption style="font-weight: bold; font-size: 1.2em; margin-bottom: 5px;">immutable/muttable 변수 비교</caption>
			</table>
			<p>
				<a href="#ex1-1">[ex1-1]</a> 9행은 4행에서 선언된 변수 a에 새로운 값 3을 할당하고 있습니다.<br>
				러스트에서 변수는 기본적으로 immutable변수로 선언되기 때문에 a는 immutable 변수이며, 
				immutable 변수에 새로운 값을 입력할 경우 에러가 발생합니다.<br>
				에러를 해결하기 위해서는 변수 이름 앞에 mut 붙여 mutable 변수로 선언해야 합니다.<br>
				let mut a: i32 = 1;<br>
			</p>
			<p>
				<a href="#ex1-2">예제 코드</a> 14~21행은 mut 변수 사용 예를 보여주고 있습니다.<br>
				5행에서 변수 aa를 mutable로 선언함과 동시에 1을 할당하고 있습니다.<br>
				이후 8행에서 새로운 값 3을 aa에 덮어쓰고 있습니다.<br>
				마지막으로 10행에서 더하기 연산을 수행하면 변수 cc에는 5가 할당됩니다.<br>
			</p>
			<Codeblock id = {"ex1-2"} range = {"3, 12"} path = {"/rust/lecture01/example/examples/ex1-2.rs"}/>
        </article>
		<hr>
		<article>
			<h1>변수 출력하기</h1>
			<p>
				선언한 변수를 출력하는 방법에 대해 알아보겠습니다.<br>
				러스트에서는 println! 매크로를 사용하여 변수를 출력할 수 있습니다.<br>
				<a href="#ex1-3">[ex1-3]</a> 8~10행은 println! 매크로를 사용하여 변수 a, b, c를 출력하고 있습니다.<br>
				println! 매크로의 첫 번째 인자는 출력하고자 하는 문자열입니다.<br>
				8행은 첫 번째 인자로 "hello rust"라는 문자열을 넣었습니다.<br>
				변수를 출력하고 싶은 경우에는 9행처럼 첫 번째 인자에 중괄호를 포함한 문자열을 넣고, 2번째 인자부터는 순서대로 중괄호에 대입할 변수들을 넣습니다.<br>
				또한 10행 처럼 첫 번째 문자열 인자의 중괄호 안에 바로 변수 이름을 적을수도 있습니다.<br>
			</p>
			<Codeblock id = {"ex1-3"} range = {"4,10"} path = {"/rust/lecture01/example/examples/ex1-3.rs"}/>
		</article>
    </section>
</main>

<!-- ex 1-1 -->

<style>
	@import '/static/css/lecture.css';
</style>