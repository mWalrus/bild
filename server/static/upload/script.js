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
    deleteFile(deleteLink)
    container.style.display = 'none'
  })
  
  container.appendChild(span)
  container.appendChild(open_svg)
  container.appendChild(delete_svg)
  
  return container
}

