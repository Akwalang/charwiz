import { parse } from './url.utils';

/*
  https://user:password@bash.org.ru:8080/story/year?offset=100&limit=10#comments
  //user:password@bash.org.ru:8080/story/year?offset=100&limit=10#comments
  ftp://user:password@bash.org.ru:8080/story/year?offset=100&limit=10#comments
  http://user:password@bash.org.ru:8080/story/year?offset=100&limit=10#comments
  https://user:password@bash.org.ru:8080/story/year?offset=100&limit=10#comments
*/

describe('parse', () => {
  it('should parse a URL correctly', () => {
    const url = 'https://www.example.com:8080/path/to/resource?query=param#fragment';

    const expected = {
      protocol: 'https',
      hostname: 'www.example.com',
      port: ':8080',
      pathname: '/path/to/resource',
      search: '?query=param',
      hash: '#fragment',
    };

    const result = parse(url);

    expect(result.hash).toBe(expected.hash);
  });
});
