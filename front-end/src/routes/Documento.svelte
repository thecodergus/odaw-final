<script>
// @ts-nocheck

    import Chart from "chart.js/auto";
    import { onMount } from "svelte";
    import Tags from "svelte-tags-input";

    import { DateInput } from "date-picker-svelte";
    let data_inicio = new Date();
    let data_fim = new Date();

    let resumoCanvas;
    const data = {
        labels: ["January", "February", "March", "April", "May"],
        datasets: [
            {
                data: [50, 60, 70, 180, 190],
                // backgroundColor: Object.values(Utils.CHART_COLORS),
            },
        ],
    };

    onMount(() => {
        const ctx = resumoCanvas.getContext("2d");
        const resumoChart = new Chart(ctx, {
            type: "doughnut",
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

<div class="container-fluid">
    <div class="row">
        <h2>Minhas contas</h2>
    </div>
    <div class="row">
        <div class="col">
            <Tags />
        </div>
        <div class="col">
            <canvas
                bind:this={resumoCanvas}
                id="resumo"
                width="600px"
                height="400px"
            />
        </div>
        <div class="col">
            <DateInput bind:value={data_inicio} format={"dd-MM-yyyy"} />
            <DateInput bind:value={data_fim} format={"dd-MM-yyyy"} />
        </div>
    </div>
    <div class="row">
        <table class="table table-striped">
            <thead>
                <tr>
                    <th scope="col">Nome da Conta</th>
                    <th scope="col">Categorias</th>
                    <th scope="col">Opções</th>
                </tr>
            </thead>
            <tbody>
                <tr>
                    <th scope="row">Remedio</th>
                    <td>
                        <span class="badge bg-secondary">Cat</span>
                    </td>
                    <td>Otto</td>
                </tr>
            </tbody>
        </table>
    </div>
</div>

<style>
    div .row {
        margin-top: 10px;
    }
</style>
