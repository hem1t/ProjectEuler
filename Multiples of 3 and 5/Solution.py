def sum35(num = 1000):
	sum = 0
	for number in range(1000):
		if number%3 == 0 or number%5 == 0:
			sum += number
	return sum


# Answer is 233168
print(sum35())