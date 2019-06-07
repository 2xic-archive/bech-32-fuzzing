
# credit

The implementation of bech32 is from https://github.com/sipa/bech32 . 

# what is this?

Fuzzer for bech32 reference implementation. Testing the bech32_decode function (wanted to rediscover the [buffer overflow](https://github.com/sipa/bech32/commit/2b0aac650ce560fb2b2a2bebeacaa5c87d7e5938) bug by cross checking implementations).

# why?

I have been playing Project Euler recently and for a task I wrote a rust library for c(so I could call rust code from c). Bech32 has a reference implementation for both c and rust. So I wanted to try to rediscover the bug by doing cross checking by fuzzing.

# did it work?

The "problem" with the c code is that the **data_len** will increment until the char *1* is found. 

~~~~
while (*data_len < input_len && input[(input_len - 1) - *data_len] != '1') {
	++(*data_len);
}
~~~~

~~~~
if (1 + *data_len >= input_len || *data_len < 6) // not problem(new version)
if (hrp_len < 1 || *data_len < 6) // problem (old version)
~~~~

If you don't have proper checks, you can get an overflow problem(as can be seen above). However, in the rust code they will check for the separator before doing anything else.

~~~~
const SEP: char = '1';
(...)
if s.find(SEP).is_none() {
	return Err(CodingError::MissingSeparator)
}
~~~~

So I set a different return value from where each function returned and if there was a missmatch, assert. **This way I was able to reproduce the bug**. You might have to prune some crashes because of different inner working of rust and c.