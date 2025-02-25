const { invoke } = window.__TAURI__.core;



  // Set up elements
const currency_form = document.querySelector("#currency-form")
const currency_swap = document.querySelector("#swap-currencies")

const currencies_element = document.querySelectorAll('.currency-dropdown')

const currency_from_name = document.querySelector("#currency-from")
const currency_from_amount = document.querySelector("#currency-from-amount")

  
const currency_to_name = document.querySelector("#currency-to")
const currency_to_amount = document.querySelector("#currency-to-amount")


function displayError(msg) {
  window.alert(msg)
}

async function getCurrencies() {
  let response = await invoke("get_currencies")
  let data = JSON.parse(response)
  let result = {}
  data.forEach(currency => {
    result[currency['iso']] = currency['name']
  });
  return result
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


function populateCurrencyLists(currencies) {

  Object.keys(currencies).forEach(iso=>{
    currencies_element.forEach(dropdown =>{
      const option = createCurrencyOption(formatCurrencyName(currencies[iso], iso), iso)
      dropdown.appendChild(option)
    })
  })

}


function handleFormSubmission(e) {
    
  e.preventDefault();

  
  invoke("get_exchange_rate", {currencyFromName:currency_from_name.value, currencyToName:currency_to_name.value})
  .then(data => {
    const exchange_rate = JSON.parse(data)
    const currency_from_rate = exchange_rate['currency_from']['value']
    const currency_to_rate = exchange_rate['currency_to']['value']

    currency_to_amount.value = Number(((currency_from_rate * currency_from_amount.value) / currency_to_rate).toFixed(5))

  })
  .catch(error => {
    displayError(error)
  })

}


function handleCurrencySwapClick(e) {

  const tmp1 = currency_from_amount.value
  const tmp2 = currency_to_amount.value

  currency_from_amount.value = tmp2
  currency_to_amount.value = tmp1

  const tmp3 = currency_from_name.value
  const tmp4 = currency_to_name.value
  
  currency_from_name.value = tmp4
  currency_to_name.value = tmp3

}


//////////
// MAIN //
//////////

const currencies = await getCurrencies();

populateCurrencyLists(currencies);

currency_form.addEventListener("submit", handleFormSubmission);

currency_swap.addEventListener("click", handleCurrencySwapClick); 



