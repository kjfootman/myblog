<script>
	function set_src(arg1, arg2) {
		document.getElementById("frame").src = "./lecture/rust/lecture" + arg2 + ".html";
	}
	
	let code = "fn main() { \
	}";
	
	function test() {
		document.getElementById("frame").contentWindow.print();
	}
</script>

<!----------------------------------------------------------------------------------------------------------------------------------- -->
<svelte:head>
	<title>this ia a my test</title>
</svelte:head>

<main class="line-numbers">
	<header>
		coding sloth
	</header>
	<section class="content">
		<nav>
			<details open>
				<summary>
					<span>Rust</span>
				</summary>
				<ul class="nav-tree">
					<li><a on:click={() => set_src('rust', '01')} href="#">[Rust01] 변수의 선언과 출력</a></li>
					<li><a on:click={() => set_src('rust', '02')} href="#">[Rust02] 변수의 종류</a></li>
					<li><a on:click={() => set_src('rust', '03')} href="#">[Rust03] 함수 사용하기</a></li>
					<li><a on:click={() => set_src('rust', '04')} href="#">[Rust04] 흐름 제어하기</a></li>
				</ul>
			</details>
		</nav>
		<iframe src="./introduction.html" id="frame" title="sub"></iframe>
	</section>
	<footer>
		footer
		<!-- <button on:click={test}>print</button> -->
	</footer>
	
	<!-- <pre class="line-numbers"><code class="language-rust">{code}</code></pre> -->
	<!-- <pre data-src="rust/lecture01/example/examples/ex1-1.rs" class="line-numbers language-rust" data-line="10"></pre> -->
</main>

<!-- -------------------------------------------------------------------------------------------------------------------------------- -->
<style>
	@import "/static/css/common.css";	

	:global(body, html) {
		margin: 0;
		height: 100%;
		background-color: azure;

		--header-height: 50px;
		--footer-height: 30px;
		--border-thickness: 2px;
		--nav-width: 300px;
	} 

	main {
		width: 100%;
		height: 100%;
	}

	header {
		width: 100%;
		height: var(--header-height);
		display: flex;
		justify-content: center;
		align-items: center;
		border-bottom: var(--border-thickness) solid black;
	}

	nav {
		width: var(--nav-width);
		height: calc(100% - 20px);
		float: left;
		padding: 10px;
		border-right: var(--border-thickness) solid black;
	}
	
	iframe {
		width: calc(100% - var(--nav-width) - var(--border-thickness) - 20px);
		height: 100%;
		margin: 0;
		padding: 0;
		border: none;
		float: left;
		/* background-color: yellow; */
	}

	footer {
		display: flex;
		justify-content: center;
		align-items: center;

		clear: both;
		height: var(--footer-height);
		border-top: var(--border-thickness) solid black;
	}

	.content {
		height: calc(100% - var(--footer-height) - var(--header-height) - calc(2 * var(--border-thickness)));;
	}

	.nav-tree {
		list-style: none;
		text-indent: -30px;
	}

	.nav-tree, li::before {
		content: "> ";
	}

	.nav-tree a, li {
		text-decoration: none;
	}
</style>