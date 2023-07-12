const TOKEN_KEY = 'bild-auth-token'

function displayMsg(msg, success = false) {
  let flash = document.querySelector('#flash-message')

  flash.classList.remove('flash-success')
  flash.classList.remove('flash-error')

  let newFlash = flash.cloneNode(true)
  // set on first render
  if (!newFlash.classList.contains('flash')) {
    newFlash.classList.add('flash')
  }
  
  if (success) {
    newFlash.classList.add('flash-success')
    newFlash.innerText = msg
  } else {
    newFlash.classList.add('flash-error')
    newFlash.innerText = 'Error: ' + msg
  }
  
  flash.parentNode.replaceChild(newFlash, flash)
}

function deleteFile(deleteLink) {
  fetch(deleteLink, {
    method: 'DELETE',
    headers: {
      authorization: `Bearer ${window.localStorage.getItem(TOKEN_KEY)}`
    }
  })
  .then(res => res.json().then(json => ({status: res.status, json})))
  .then(({status, json}) => {
    displayMsg(json.message, status === 200)
  })
}
