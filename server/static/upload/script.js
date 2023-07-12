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

async function loadHistory() {
  const token = window.localStorage.getItem(TOKEN_KEY)

  const res = await fetch('/history', {
    method: 'GET',
    headers: {
      authorization: `Bearer ${token}`
    },
    cache: 'no-cache'
  })

  const json = await res.json();  

  if (res.status === 200) {
    let linkContainer = document.getElementById('link-container')
    // hide all potential uploads already visible in the upload list
    for (const child of linkContainer.children) {
      child.classList.add('hidden')
    }

    for (const link of json.history) {
      let linkElement = createLinkElement(link)
      // we can use this to remove all history elements when the user decides
      linkElement.classList.add('history-upload')
      linkContainer.appendChild(linkElement)
    }

    document.getElementById('history').classList.add('hidden')
    document.getElementById('hide-history').classList.remove('hidden')
  } else {
    displayMsg(json.message)
  }
}

async function hideHistory() {
  const linkContainer = document.getElementById('link-container')
  const children = [...linkContainer.children]
  for (const child of children) {
    if (child.classList.contains('history-upload')) {
      linkContainer.removeChild(child)
    }
  }

  // unhide all potentially hidden uploads from before
  for (const child of linkContainer.children) {
    child.classList.remove('hidden')
  }

  document.getElementById('history').classList.remove('hidden')
  document.getElementById('hide-history').classList.add('hidden')
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
          const { link, delete_link, size, created } = response 
          if (!link || !delete_link) {
            throw new Error(response.message ?? 'Unknown error when uploading')
          }
          resolve({link, deleteLink: delete_link, size, created})
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
  const {link, deleteLink, size, created} = links
  
  let container = document.createElement('div')
  container.classList.add('link-copy-container')

  let fileName = link.slice(link.lastIndexOf('/') + 1)
  container.id = fileName

  let tmp = document.getElementsByTagName('template')[0]
  let linkContainer = tmp.content.cloneNode(true)
  let div = linkContainer.children[0]
  let children = div.children

  let imgTag = children[0]
  let linkTag = children[1]

  // TODO: update these when we have the history metadata implemented
  let dimsTag = children[2].children[0]
  let sizeTag = children[2].children[1]
  let uploadedTag = children[2].children[2]

  imgTag.src = link
  imgTag.addEventListener('click', () => {
    window.open(link, '_blank')
  })

  // perfect solution
  imgTag.onload = () => dimsTag.innerText += ` ${imgTag.naturalWidth}x${imgTag.naturalHeight}`

  linkTag.innerText = link
  
  linkTag.addEventListener('click', () => {
    navigator.clipboard.writeText(link)
    displayMsg('Copied link to clipboard', true)
  })

  sizeTag.innerText += humanSize(size)
  uploadedTag.innerText += humanDate(created)
  
  
  let open_svg = createSvg([SVG_PATH_OPEN_LINK], 'Open in new tab')
  open_svg.classList.add('open-external')
  open_svg.addEventListener('click', () => {
    window.open(link, '_blank')
  })

  let delete_svg = createSvg([SVG_PATH_TRASH_OUTER, SVG_PATH_TRASH_INNER], 'Delete file')
  delete_svg.classList.add('delete-file')
  delete_svg.addEventListener('click', () => {
    deleteFile(deleteLink)
    div.style.display = 'none'
  })

  div.appendChild(open_svg)
  div.appendChild(delete_svg)

  return div
}

function humanSize(bytes) {
  if (bytes < 1024) {
    return bytes + "B"
  } else if (bytes < 1024 * 1024) {
    return (bytes / 1024).toFixed(1) + "kB"
  } else {
    return (bytes / 1024 / 1024).toFixed(1) + "mB"
  }
}

function humanDate(d) {
  return new Date(d.secs_since_epoch * 1000).toDateString()
}

