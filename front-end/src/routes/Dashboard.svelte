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

<nav class="navbar navbar-expand-lg fixed-top nav-cor">
    <div class="container-fluid">
        <a class="navbar-brand" href="#"
            >Sistema de Controle Financeiro Pessoal</a
        >
        <button
            class="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarNavAltMarkup"
            aria-controls="navbarNavAltMarkup"
            aria-expanded="false"
            aria-label="Toggle navigation"
        >
            <span class="navbar-toggler-icon"></span>
        </button>
        <div class="collapse navbar-collapse" id="navbarNavAltMarkup">
            <div class="navbar-nav">
                <a class="nav-link active" aria-current="page" href="#"
                    >Dashboard</a
                >
                <a class="nav-link" href="#">Minhas Contas</a>
                <a class="nav-link" href="#">Minhas Receitas</a>
                <a class="nav-link" href="#">Meu Perfil</a>
                <a class="nav-link sair-link" href="#">Sair</a>
            </div>
        </div>
    </div>
</nav>

<div class="container">
    <h2>Resumo</h2>
    <canvas bind:this={resumoCanvas} id="resumo" width="800px" height="600px"
    ></canvas>
</div>

<style>
    .nav-cor {
        background-color: #ffc02f;
    }

    .sair-link {
        position: absolute;
        right: 0;
    }
</style>
