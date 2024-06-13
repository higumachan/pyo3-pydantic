from pydantic import BaseModel


class Pet(BaseModel):
    name: str
    age: int
    weight: float
    is_vaccinated: bool
