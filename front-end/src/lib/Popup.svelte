<script>
    export let open = false;
    export let title = "";
    export let message = "";
    export let showBackdrop = true;
    export let onClosed;

    const modalClose = () => {
        open = false;
        if (onClosed) {
            onClosed();
        }
    };
</script>

{#if open}
    <div
        class="modal"
        id="sampleModal"
        tabindex="-1"
        role="dialog"
        aria-labelledby="sampleModalLabel"
        aria-hidden={false}
    >
        <div class="modal-dialog" role="document">
            <div class="modal-content">
                <div class="modal-header">
                    <h5 class="modal-title" id="sampleModalLabel">
                        {title}
                    </h5>
                </div>
                <div class="modal-body">{message}</div>
                <div class="modal-footer">
                    <button
                        type="button"
                        class="btn btn-danger"
                        data-dismiss="modal"
                        on:click={modalClose}>Close</button
                    >
                </div>
            </div>
        </div>
    </div>
    {#if showBackdrop}
        <div class="modal-backdrop show" />
    {/if}
{/if}

<style>
    .modal {
        display: block;
    }
</style>
