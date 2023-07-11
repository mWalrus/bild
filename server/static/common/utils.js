const TOKEN_KEY = 'bild-auth-token'
const SVG_PATH_TRASH_OUTER = 'M20 2h-4v-.85C16 .52 15.48 0 14.85 0h-5.7C8.52 0 8 .52 8 1.15V2H4c-1.1 0-2 .9-2 2 0 .74.4 1.38 1 1.73v14.02C3 22.09 4.91 24 7.25 24h9.5c2.34 0 4.25-1.91 4.25-4.25V5.73c.6-.35 1-.99 1-1.73 0-1.1-.9-2-2-2zm-1 17.75c0 1.24-1.01 2.25-2.25 2.25h-9.5C6.01 22 5 20.99 5 19.75V6h14v13.75z'
const SVG_PATH_TRASH_INNER = 'M8 20.022c-.553 0-1-.447-1-1v-10c0-.553.447-1 1-1s1 .447 1 1v10c0 .553-.447 1-1 1zm8 0c-.553 0-1-.447-1-1v-10c0-.553.447-1 1-1s1 .447 1 1v10c0 .553-.447 1-1 1zm-4 0c-.553 0-1-.447-1-1v-10c0-.553.447-1 1-1s1 .447 1 1v10c0 .553-.447 1-1 1z'
const SVG_PATH_OPEN_LINK = 'M 5 3 C 3.9069372 3 3 3.9069372 3 5 L 3 19 C 3 20.093063 3.9069372 21 5 21 L 19 21 C 20.093063 21 21 20.093063 21 19 L 21 12 L 19 12 L 19 19 L 5 19 L 5 5 L 12 5 L 12 3 L 5 3 z M 14 3 L 14 5 L 17.585938 5 L 8.2929688 14.292969 L 9.7070312 15.707031 L 19 6.4140625 L 19 10 L 21 10 L 21 3 L 14 3 z'

function createSvg(paths, title) {
  const iconSvg = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
  iconSvg.setAttribute('viewBox', '0 0 24 24')
  
  // allows for more complex svgs to be created
  for (let path of paths) {
    const iconPath = document.createElementNS(
      'http://www.w3.org/2000/svg',
      'path'
    )
    iconPath.setAttribute('d', path)
    const iconTitle = document.createElement('title')
    iconTitle.id = 'title'
    iconTitle.innerText = title
    iconSvg.appendChild(iconTitle)
    iconSvg.appendChild(iconPath)
  }

  return iconSvg
}

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
