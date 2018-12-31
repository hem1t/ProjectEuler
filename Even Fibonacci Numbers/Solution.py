def fibsum():
	result = 0
	previous = 0
	fib1 = 0
	fib2 = 1
	while fib2 < 4000000:
		previous = fib2
		fib2 += fib1

		if fib2%2 == 0:
			result += fib2
		
		fib1 = previous
		
	print(result)

# Answer is: 4613732
fibsum()