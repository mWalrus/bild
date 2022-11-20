const TOKEN_KEY = 'bild-auth-token'
const SVG_PATH_TRASH_OUTER = 'M20 2h-4v-.85C16 .52 15.48 0 14.85 0h-5.7C8.52 0 8 .52 8 1.15V2H4c-1.1 0-2 .9-2 2 0 .74.4 1.38 1 1.73v14.02C3 22.09 4.91 24 7.25 24h9.5c2.34 0 4.25-1.91 4.25-4.25V5.73c.6-.35 1-.99 1-1.73 0-1.1-.9-2-2-2zm-1 17.75c0 1.24-1.01 2.25-2.25 2.25h-9.5C6.01 22 5 20.99 5 19.75V6h14v13.75z'
const SVG_PATH_TRASH_INNER = 'M8 20.022c-.553 0-1-.447-1-1v-10c0-.553.447-1 1-1s1 .447 1 1v10c0 .553-.447 1-1 1zm8 0c-.553 0-1-.447-1-1v-10c0-.553.447-1 1-1s1 .447 1 1v10c0 .553-.447 1-1 1zm-4 0c-.553 0-1-.447-1-1v-10c0-.553.447-1 1-1s1 .447 1 1v10c0 .553-.447 1-1 1z'
const SVG_PATH_OPEN_LINK = 'M 5 3 C 3.9069372 3 3 3.9069372 3 5 L 3 19 C 3 20.093063 3.9069372 21 5 21 L 19 21 C 20.093063 21 21 20.093063 21 19 L 21 12 L 19 12 L 19 19 L 5 19 L 5 5 L 12 5 L 12 3 L 5 3 z M 14 3 L 14 5 L 17.585938 5 L 8.2929688 14.292969 L 9.7070312 15.707031 L 19 6.4140625 L 19 10 L 21 10 L 21 3 L 14 3 z'

function dragOverHandler(e) {
  e.preventDefault()
}

function dropHandler(e) {
  e.preventDefault()
  let file = e.dataTransfer.files[0]
  handleFile(file)
}

function pasteHandler(e) {
  e.preventDefault()
  // TODO: this doesn't work in the current version
  let items = (e.clipboardData || e.originalEvent.clipboardData).items
  console.log(JSON.stringify(items))
}

function load() {
  const token = window.localStorage.getItem(TOKEN_KEY)
  if (token === null) {
    // remove any lingering event listeners
    document.removeEventListener('dragover', dragOverHandler)
    document.removeEventListener('drop', dropHandler)

    // apply the appropriate classes
    document.getElementById('upload-container').classList.add('hidden')
    document.getElementById('token-container').classList.remove('hidden')

    // allows the user to paste and press enter
    document.getElementById('token-input').addEventListener('keypress', (e) => {
      if (e.key === 'Enter') {
        e.preventDefault()
        document.getElementById('submit-token-btn').click()
      }
    })
  } else {
    addEventListeners()
  }
}

function addEventListeners() {
  let fileInput = document.getElementById('file-input')
  fileInput.focus()
  fileInput.addEventListener('change', inputHandler, false)

  document.addEventListener('dragover', dragOverHandler, false)
  document.addEventListener('drop', dropHandler, false)

  document.querySelector('#clear-token').addEventListener('click', () => {
    window.localStorage.removeItem(TOKEN_KEY)
    load()
  })
}

async function setToken() {
  let tokenInput = document.getElementById('token-input').value
  if (tokenInput.length === 0) {
    displayMsg('Invalid token')
    return
  }
  
  const res = await fetch('/token-validation', {
    method: 'POST',
    headers: {
      authorization: `Bearer ${tokenInput}`
    },
    cache: 'no-cache'
  })
  
  if (res.status !== 202) {
    let json = await res.json()
    displayMsg(json.message)
    return
  }
  window.localStorage.setItem(TOKEN_KEY, tokenInput)

  document.getElementById('upload-container').classList.remove('hidden')
  document.getElementById('token-container').classList.add('hidden')

  addEventListeners()
}

async function inputHandler(e) {
  e.preventDefault()
  let file = e.target.files[0]
  handleFile(file)
}

async function handleFile(file) {
  try {
    let links = await uploadFile(file)
    
    displayMsg('Uploaded ' + file.name, true)

    let linkElement = createLinkElement(links)
    let linkContainer = document.getElementById('link-container')
    linkContainer.appendChild(linkElement)
  } catch (e) {
    displayMsg(e.message)
  }
}

function uploadFile(file) {
  return new Promise((resolve, reject) => {
    
    const formData = new FormData()
    formData.append('data', file)

    let xhr = new XMLHttpRequest()
    
    let prevReadyState = 0
    
    xhr.onreadystatechange = () => {
      if (xhr.readyState === 4) {
        try {
          // the request is done but failed to send
          if (prevReadyState === 1) {
            throw new Error('File might be too large')
          }

          if (!xhr.responseText) {
            throw new Error('Body is missing in response')
          }

          let response = JSON.parse(xhr.responseText)
          const { link, delete_link } = response 
          if (!link || !delete_link) {
            throw new Error(response.message ?? 'Unknown error when uploading')
          }
          resolve({link, deleteLink: delete_link})
        } catch (e) {
          reject(e)
        }    
      }
      
      // keep track of last state change
      prevReadyState = xhr.readyState
    }

    xhr.addEventListener('error', () => {
      reject(new Error('Request failed'))
    })

    xhr.addEventListener('abort', () => {
      reject(new Error('Request aborted'))
    })
    
    xhr.open('POST', '/upload')
    xhr.setRequestHeader('Authorization', `Bearer ${localStorage.getItem(TOKEN_KEY)}`)
    xhr.send(formData)
  })
}

function createLinkElement(links) {
  const {link, deleteLink} = links

  
  let container = document.createElement('div')
  container.classList.add('link-copy-container')

  let fileName = link.slice(link.lastIndexOf('/') + 1)
  container.id = fileName

  let span = document.createElement('span')
  span.classList.add('link-copy')
  span.innerText = link
  
  span.addEventListener('click', () => {
    navigator.clipboard.writeText(link)
    displayMsg('Copied link to clipboard', true)
  })
  
  
  let open_svg = createSvg([SVG_PATH_OPEN_LINK])
  open_svg.classList.add('open-external')
  open_svg.addEventListener('click', () => {
    window.open(link, '_blank')
  })

  let delete_svg = createSvg([SVG_PATH_TRASH_OUTER, SVG_PATH_TRASH_INNER])
  delete_svg.classList.add('delete-file')
  delete_svg.addEventListener('click', () => {
    deleteFile(deleteLink, fileName)
    container.style.display = 'none'
  })
  
  container.appendChild(span)
  container.appendChild(open_svg)
  container.appendChild(delete_svg)
  
  return container
}

function deleteFile(link) {
    fetch(link, {
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

function createSvg(paths) {
  const iconSvg = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
  iconSvg.setAttribute('viewBox', '0 0 24 24')
  
  // allows for more complex svgs to be created
  for (let path of paths) {
    const iconPath = document.createElementNS(
      'http://www.w3.org/2000/svg',
      'path'
    )
    iconPath.setAttribute('d', path)
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
