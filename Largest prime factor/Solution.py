def isprime(num):
	if num%2 == 0:
		return False

	if num == 1:
		return False

	if num == 3 or num == 5 or num == 9:
		return True

	for i in range(2, 9):
		if num % i == 0:
			return False

	return True


while True:
	print(isprime(int(input(":"))))

