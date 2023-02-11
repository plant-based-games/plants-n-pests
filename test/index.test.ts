import { hello } from '../src/index'

describe('hello', () => {
  it('returns a greeting', () => {
    expect(hello()).toBe('Hello world!\n')
  })
})
