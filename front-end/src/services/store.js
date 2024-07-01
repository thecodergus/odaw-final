
export const usuario = {
    get: () => JSON.parse(localStorage.getItem("trabalho_odaw_login")),
    set: (v) => localStorage.setItem("trabalho_odaw_login", JSON.stringify(v))
}