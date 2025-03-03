const { invoke } = window.__TAURI__.core;


  // Set up elements
const currency_form = document.querySelector("#currency-form")
const currency_swap = document.querySelector("#swap-currencies")

const currencies_element = document.querySelectorAll('.currency-dropdown')

const currency_from_name = document.querySelector("#currency-from")
const currency_from_amount = document.querySelector("#currency-from-amount")

  
const currency_to_name = document.querySelector("#currency-to")
const currency_to_amount = document.querySelector("#currency-to-amount")

// Display an error
function displayError(msg) {
  console.log("Displaying error:", msg);
  alert("An issue occured when performing the requested action: ", msg)
}


// Get a list of currencies available
async function getCurrencies() {

    return invoke("get_currencies").then(response => {    
      let data = JSON.parse(response)
      console.log(data);
      
      data = data['payload']
      let result = {}
      data.forEach(currency => {
        result[currency['iso']] = currency['name']
      })

      return result
    })

    .catch(err => {
      throw err
    })
}


// The format a currency is displayed in
function formatCurrencyName(name, ISO) {
  return `${name} (${ISO})`
}


// Creates an option element with the given values for use in currency selection list
function createCurrencyOption(name, value) {
  const option = document.createElement('option')
  
  option.textContent = name
  option.setAttribute('value', value)
  
  return option
}


// Initialization of currency selection list
function populateCurrencyLists() {
  getCurrencies().then(data => {

    console.log(data);
    

    let currencies = data
    Object.keys(currencies).forEach(iso=>{
      currencies_element.forEach(dropdown =>{
        const option = createCurrencyOption(formatCurrencyName(currencies[iso], iso), iso)
        dropdown.appendChild(option)
      })
    })

  }, err => {
    // setTimeout(displayError(err), 10)
    setTimeout(alert(err), 10)
  })


}


// Handle the conversion request
function handleFormSubmission(e) {
  e.preventDefault();
  
  invoke("get_exchange_rate", {currencyFromName:currency_from_name.value, currencyToName:currency_to_name.value})
  .then(data => {
    const exchange_rate = JSON.parse(data)['payload']
    const currency_from_rate = exchange_rate['currency_from']['value']
    const currency_to_rate = exchange_rate['currency_to']['value']

    currency_to_amount.value = Number(((currency_from_rate * currency_from_amount.value) / currency_to_rate).toFixed(5))

  })
  .catch(error => {
    displayError(error)
  })

}


// Swap currently selected currencies
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

populateCurrencyLists();

currency_form.addEventListener("submit", handleFormSubmission);

currency_swap.addEventListener("click", handleCurrencySwapClick); 



