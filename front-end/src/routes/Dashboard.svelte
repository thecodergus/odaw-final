<script>
    import Chart from "chart.js/auto";
    import { onMount } from "svelte";

    let resumoCanvas;
    const data = {
        labels: ["Janeiro", "Fevereiro", "Março", "Abril", "Maio"],
        datasets: [
            {
                label: "Contas",
                data: [-20, -30, -20, -30, -40],
                borderColor: "red",
                backgroundColor: "rgba(255, 0, 0, 0.5)",
                borderWidth: 2,
                borderRadius: Number.MAX_VALUE,
                borderSkipped: false,
            },
            {
                label: "Receitas",
                data: [20, 30, 20, 30, 40],
                borderColor: "blue",
                backgroundColor: "rgba(0, 0, 255, 0.5)",
                borderWidth: 2,
                borderRadius: 5,
                borderSkipped: false,
            },
        ],
    };

    onMount(() => {
        const ctx = resumoCanvas.getContext("2d");
        const resumoChart = new Chart(ctx, {
            type: "bar",
            data: data,
            options: {
                responsive: true,
                plugins: {
                    legend: {
                        position: "top",
                    },
                    title: {
                        display: true,
                        text: "Chart.js Bar Chart",
                    },
                },
            },
        });
    });
</script>

<div class="container">
    <h2>Resumo</h2>
    <canvas bind:this={resumoCanvas} id="resumo" width="800px" height="600px"
    ></canvas>
</div>

<style>
</style>
