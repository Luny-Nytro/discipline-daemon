export function JsonResponse(json: string) {
  return new Response(json, {
    status: 200,
    statusText: "OK",
    headers: {
      "Content-Type": "application/json",
      "Content-Length": json.length.toString(),
    }
  })
}

export function NotFound() {
  const status = "Not Found"
  return new Response(status, {
    status: 404,
    statusText: status,
    headers: { 
      "Content-Type": "text/plain",
      "Content-Length": status.length.toString(),
    }
  })
}

export function OkResponse(message: string) {
  return new Response(message, {
    status: 200,
    statusText: "OK",
    headers: {
      "Content-Type": "text/plain",
      "Content-Length": message.length.toString(), 
    }    
  })
}

export function BadRequest(reason: string) {
  return new Response(reason, {
    status: 400,
    statusText: "Bad Request",
    headers: {
      "Content-Type": "text/plain",
      "Content-Length": reason.length.toString(), 
    }
  })
}

export function HTMLResponse(html: string) {
  return new Response(html, {
    status: 200,
    statusText: "OK",
    headers: {
      "Content-Type": "text/html",
      "Content-Length": html.length.toString(),
    }
  })
}

export function InternalServerError(error: string) {
  return new Response(error, {
    status: 500,
    statusText: "Internal Server Error",
    headers: {
      "Content-Type": "text/plain",
      "Content-Length": error.length.toString(), 
    }
  })
}
