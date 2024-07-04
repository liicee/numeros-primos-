n = int(input("Digite um numero:  "))
contador = 0

for c in range(1, n +1):
    if n % c == 0:
        contador +=1
if contador == 2:
    print(n, " é um numero primo")   
else:
    print(n, "não é primo")
