export function read_file(path: string) {
  const request = new XMLHttpRequest();
  request.open('GET', path, false);  // `false` makes the request synchronous
  request.send(null);

  let response = "";
  
  if (request.status === 200) {
    response = request.responseText;
  } else {
    console.error(`Failed to read_file(${path})`);
  }

  return response;
}
