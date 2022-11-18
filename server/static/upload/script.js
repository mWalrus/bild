const TOKEN_KEY = 'bild-auth-token'
const EXT_URL_SVG = new DOMParser().parseFromString('<svg fill="#D8DEE9" xmlns="http://www.w3.org/2000/svg"  viewBox="0 0 24 24" width="24px" height="24px"><path d="M 5 3 C 3.9069372 3 3 3.9069372 3 5 L 3 19 C 3 20.093063 3.9069372 21 5 21 L 19 21 C 20.093063 21 21 20.093063 21 19 L 21 12 L 19 12 L 19 19 L 5 19 L 5 5 L 12 5 L 12 3 L 5 3 z M 14 3 L 14 5 L 17.585938 5 L 8.2929688 14.292969 L 9.7070312 15.707031 L 19 6.4140625 L 19 10 L 21 10 L 21 3 L 14 3 z"/></svg>', 'text/html').body.firstChild

function dragOverHandler(e) {
  e.preventDefault()
}

function dropHandler(e) {
  e.preventDefault()
  // something else
  let file = e.dataTransfer.files[0]
  handleFile(file)
}

function pasteHandler(e) {
  e.preventDefault()
  let items = (e.clipboardData || e.originalEvent.clipboardData).items
  console.log(JSON.stringify(items))
}

function load() {
  const token = window.localStorage.getItem(TOKEN_KEY)
  if (token === null) {
    document.getElementById('upload-container').classList.add('hidden')
    document.getElementById('token-container').classList.remove('hidden')
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
}

function setToken() {
  let tokenInput = document.getElementById('token-input').value
  if (tokenInput.length === 0) {
    displayError('Invalid token')
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
    let link = await uploadFile(file)
    let linkElement = createLinkElement(link)
    let linkContainer = document.getElementById('link-container')
    linkContainer.appendChild(linkElement)
  } catch (e) {
    displayError(e.message)
  }
}

function uploadFile(file) {
  return new Promise((resolve, reject) => {
    
    const formData = new FormData()
    formData.append('data', file)

    let xhr = new XMLHttpRequest()
    
    xhr.onreadystatechange = () => {
      if (xhr.readyState === 4) {
        try {
          if (!xhr.responseText) {
            throw new Error('Response body is missing')
          }
          let response = JSON.parse(xhr.responseText)
          const { link } = response 
          if (!link) {
            throw new Error(response.message || 'Unknown error when uploading')
          }
          resolve(link)
        } catch (e) {
          reject(e)
        }    
      }
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

function createLinkElement(link) {
  let container = document.createElement('div')
  container.classList.add('link-copy-container')

  let span = document.createElement('span')
  span.classList.add('link-copy')
  span.innerText = link
  
  span.addEventListener('click', () => {
    navigator.clipboard.writeText(link)
    // TODO: flash to the user that the text has been copied.
  })
  
  container.appendChild(span)
  
  let svg = createSvg()
  
  svg.addEventListener('click', () => {
    window.open(link, '_blank')
  })
  
  container.appendChild(svg) 
  
  return container
}

function createSvg() {
  const iconSvg = document.createElementNS('http://www.w3.org/2000/svg', 'svg')
  const iconPath = document.createElementNS(
    'http://www.w3.org/2000/svg',
    'path'
  )
  iconSvg.setAttribute('viewBox', '0 0 24 24')
  iconPath.setAttribute('d', 'M 5 3 C 3.9069372 3 3 3.9069372 3 5 L 3 19 C 3 20.093063 3.9069372 21 5 21 L 19 21 C 20.093063 21 21 20.093063 21 19 L 21 12 L 19 12 L 19 19 L 5 19 L 5 5 L 12 5 L 12 3 L 5 3 z M 14 3 L 14 5 L 17.585938 5 L 8.2929688 14.292969 L 9.7070312 15.707031 L 19 6.4140625 L 19 10 L 21 10 L 21 3 L 14 3 z')
  iconSvg.appendChild(iconPath)
  return iconSvg
}

function displayError(msg) {
  let flash = document.querySelector('#flash-message')
  let newFlash = flash.cloneNode(true)
  // set on first render
  if (!newFlash.classList.contains('flash')) {
    newFlash.classList.add('flash')
  }
  newFlash.innerText = 'Error: ' + msg
  flash.parentNode.replaceChild(newFlash, flash)
}
