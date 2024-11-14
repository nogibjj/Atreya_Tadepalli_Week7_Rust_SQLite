import time

def add_numbers(x,y):
    num_12=x
    num2=y
    z=1
    for i in range(num_12,num2+1,1):
        z=z+i
    return z


start=time.perf_counter()
sum=add_numbers(1,100000)
end=time.perf_counter()
print(f"The sum of all of these numbers is {sum}")
print(f"The time taken is {10**6*(end - start)} microseconds")
