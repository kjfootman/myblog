<script>
	function set_src(arg1, arg2) {
		document.getElementById("frame").src = "/lecture/rust/lecture" + arg2 + "/";
	}
</script>

<!-- -------------------------------------------------------------------------------------------------------------------------------- -->
<svelte:head>
	<title>this ia a my test</title>
</svelte:head>

<main>
	<header>
		header
	</header>
	<section class="content">
		<nav>
			<details open>
				<summary>
					<span>Rust</span>
				</summary>
				<ul class="nav-tree">
					<li><a on:click={() => set_src('rust', '01')} href="/">lecture01</a></li>
					<li><a on:click={() => set_src('rust', '02')} href="/">lecture02</a></li>
				</ul>
			</details>
		</nav>
		<iframe src="/introduction" id="frame" title="sub"></iframe>
	</section>
	<footer>footer</footer>
</main>

<!-- -------------------------------------------------------------------------------------------------------------------------------- -->
<style>
	:global(body, html) {
		margin: 0;
		height: 100%;
		background-color: azure;

		--header-height: 70px;
		--footer-height: 50px;
		--border-thickness: 2px;
		--nav-width: 350px;
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

		background-color: yellow;
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