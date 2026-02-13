type Anatomy = {
  protocol: string | null; // http, https, ftp, etc.
  auth: string | null; // user:password
  host: string | null; // domain name or IP address
  port: string | null; // port number
  path: string | null; // path to the resource
  search: string | null; // query parameters
  hash: string | null; // fragment identifier
};

const extractProtocol = (url: string): [string | null, string] => {
  const idx = url.indexOf('//');

  if (idx === -1) return [null, url];

  const protocol = url.slice(0, idx + 2);
  const rest = url.slice(idx + 2);

  return [protocol.replace(/:?\/\/$/, '') || null, rest];
};

const extractAuth = (url: string): [string | null, string] => {
  const idx = url.indexOf('@');

  return idx !== -1 ? [url.slice(0, idx), url.slice(idx + 1)] : [null, url];
};

const extractPort = (url: string): [string | null, string] => {
  const idx = url.indexOf(':');

  return idx !== -1 ? [url.slice(idx + 1), url.slice(0, idx)] : [null, url];
};

const extractPath = (url: string): [string, string] => {
  const idx = url.indexOf('/');

  return idx !== -1 ? [url.slice(idx), url.slice(0, idx)] : ['/', url];
};

const extractSearch = (url: string): [string | null, string] => {
  const idx = url.indexOf('?');

  return idx !== -1 ? [url.slice(idx + 1), url.slice(0, idx)] : [null, url];
};

const extractHash = (url: string): [string | null, string] => {
  const idx = url.indexOf('#');

  return idx !== -1 ? [url.slice(idx + 1), url.slice(0, idx)] : [null, url];
};

export const parseUrl = (url: string): Anatomy => {
  var rest = url;

  var { 0: protocol, 1: rest } = extractProtocol(rest);
  var { 0: hash, 1: rest } = extractHash(rest);
  var { 0: search, 1: rest } = extractSearch(rest);
  var { 0: path, 1: rest } = extractPath(rest);
  var { 0: auth, 1: rest } = extractAuth(rest);
  var { 0: port, 1: host } = extractPort(rest);

  return { protocol, auth, host, port, path, search, hash };
};

export const parseSearch = (search: string | null): [string, string][] => {
  const params: [string, string][] = [];

  if (!search) return params;

  search.split('&').forEach((param) => {
    const idx = param.indexOf('=');

    const key = idx !== -1 ? param.slice(0, idx) : param;
    const value = idx !== -1 ? param.slice(idx + 1) : '';

    params.push([key, value]);
  });

  return params;
};

export const joinUrl = (url: Anatomy): string => {
  let result = '';

  if (url.protocol) result += url.protocol + '://';
  if (url.auth) result += url.auth + '@';
  if (url.host) result += url.host;
  if (url.port) result += ':' + url.port;
  if (url.path) result += url.path;
  if (url.search) result += '?' + url.search;
  if (url.hash) result += '#' + url.hash;

  return result;
};
