<script>
    import Chart from "chart.js/auto";
    import { onMount } from "svelte";
    import { get_documentos } from "../services/api";
    import { usuario } from "../services/store";
    import Popup from "../lib/Popup.svelte";

    let resumoCanvas;
    let popUpMostrar = false;
    let popUpTitle = "";
    let popUpMensagem = "";

    onMount(() => {
        // Arrumando datas
        const data_atual = new Date();
        const anterior_mes_2 = new Date(data_atual);
        anterior_mes_2.setMonth(anterior_mes_2.getMonth() - 2);
        const anterior_mes_1 = new Date(data_atual);
        anterior_mes_1.setMonth(anterior_mes_1.getMonth() - 1);
        const proximo_mes_1 = new Date(data_atual);
        proximo_mes_1.setMonth(proximo_mes_1.getMonth() + 1);
        const proximo_mes_2 = new Date(data_atual);
        proximo_mes_2.setMonth(proximo_mes_2.getMonth() + 2);
        const meses = new Intl.DateTimeFormat("pt-BR", { month: "long" });

        get_documentos(usuario.get().id)
            .then((res) => {
                console.log(res.data);
                const data = {
                    labels: [
                        meses.format(anterior_mes_2),
                        meses.format(anterior_mes_1),
                        meses.format(data_atual),
                        meses.format(proximo_mes_1),
                        meses.format(proximo_mes_2),
                    ],
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
            })
            .catch((err) => {
                console.error(err);
            });
    });
</script>

<div class="container">
    <h2>Resumo</h2>
    <canvas bind:this={resumoCanvas} id="resumo" width="800px" height="600px"
    ></canvas>
</div>
<Popup
    message={popUpMensagem}
    open={popUpMostrar}
    title={popUpTitle}
    onClosed={() => {
        popUpMostrar = false;
    }}
/>

<style>
</style>
