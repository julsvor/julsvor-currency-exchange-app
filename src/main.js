const { invoke } = window.__TAURI__.core;

async function getCurrencies() {
  let response = await invoke("get_currencies")

  response = JSON.parse(response)

  return response['currencies']
}



function formatCurrencyName(name, ISO) {
  return `${name} (${ISO})`
}



function createCurrencyOption(name, value) {
  const option = document.createElement('option')
  
  option.textContent = name
  option.setAttribute('value', value)
  
  return option
}


async function getExchangeRate(from, to) {
const exchange_rate = await invoke("get_exchange_rate", {currencyFrom:from, currencyTo:to})
return JSON.parse(exchange_rate)
}


window.addEventListener("DOMContentLoaded", async () => {

  // Set up elements
  const currency_formE = document.querySelector("#currency-form")
  const currency_swapE = document.querySelector("#swap-currencies")

  const currencies_element = document.querySelectorAll('.currency-dropdown')
  const currencies = await getCurrencies()

  const currency_fromE = document.querySelector("#currency-from")
  const currency_from_amountE = document.querySelector("#currency-from-amount")

  
  const currency_toE = document.querySelector("#currency-to")
  const currency_to_amountE = document.querySelector("#currency-to-amount")

  // Populate currencies list
  Object.keys(currencies).forEach(iso=>{
    currencies_element.forEach(dropdown =>{
      const option = createCurrencyOption(formatCurrencyName(currencies[iso], iso), iso)
      dropdown.appendChild(option)
    })
  })


  currency_formE.addEventListener("submit", async (e) => {
    e.preventDefault();
    const currency_amount = currency_from_amountE.value
    const exchange_rate = await getExchangeRate(currency_fromE.value, currency_toE.value)
    currency_to_amountE.value = Number((exchange_rate['result'][currency_toE.value] * currency_amount).toFixed(5))
  });

  currency_swapE.addEventListener("click", async function() {
    const tmp1 = currency_from_amountE.value
    const tmp2 = currency_to_amountE.value
    
    const tmp3 = currency_fromE.value
    const tmp4 = currency_toE.value

    currency_from_amountE.value = tmp2
    currency_to_amountE.value = tmp1

    currency_fromE.value = tmp4
    currency_toE.value = tmp3

  })

});
