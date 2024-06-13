import simple_example

import simple_example_py

if __name__ == '__main__':
    pet = simple_example_py.Pet(name='Mittens', age=2, weight=10.5, is_vaccinated=True)
    print(pet)
    pet_2times = simple_example.pet_age_2times(pet)
    print(pet_2times)
