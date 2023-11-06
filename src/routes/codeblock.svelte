<script>
    export let path = NaN;
    export let id = '';
    export let range = '0,0'
</script>

<div id={id} class="codeblock">
    {#if path != NaN}
        <pre style="width: 100%;" data-src="{path}" data-range={range} class="line-numbers language-rust"></pre>
    {:else}    
        <p>cannot find code from {path}</p>
    {/if}
    <div class="codecaption">[예제 {id}]</div>
</div>

<style>
    .codeblock {
        margin: 20px auto;
        padding: 0;
        width: 75%;
        height: 100%;
        font-size: 13px;
    }

    .codecaption {
        width: 100%;
        text-align: center;
        font-weight: bold;
    }
</style>