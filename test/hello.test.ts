import { hello } from '../src/hello'

describe('hello', () => {
  it('returns a greeting', () => {
    expect(hello()).toBe('Hello world!\n')
  })
})
