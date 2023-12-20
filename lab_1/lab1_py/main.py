import abc
from math import py

class figure(abc.ABC):
    def square(self):
        pass

    def repr(self):
        pass

    def get_name(self):
        pass

class color:
    def __init__(self, color_='Красный'):
        self.color = color_

    def __str__(self):
        return self.color



class rectangle(abstract.figure):
    def __init__(self, height = 0, length = 0, color_= 'Красный'):
        self._length = length
        self.__height = height
        self._color = color.color(color_)
        self._name = 'Прямоугольник'

    def square(self):
        res = self.__height * self._length
        return res

    def get_name(self):
        return self._name

    def get_color(self):
        return self._color

    def repr(self):
        print(f'Название фигуры: {self.get_name()}\n'
              f'Цвет фигуры: {self.get_color()}\n'
              f'Площадь фигуры: {self.square()}\n'
              + '\n'
              )
        



class circle(abstract.figure):
    def __init__(self, rad = 10, color_ = 'Красный '):
        self.__name = 'Круг'
        self.__color = color.color(color_)
        self.__rad = rad

    def square(self):
        res = pi * self.__rad ** 2
        return res

    def get_name(self):
        return self.__name

    def get_color(self):
        return self.__color

    def repr(self):
        print(f'Название фигуры: {self.get_name()}\n'
              f'Цвет фигуры: {self.get_color()}\n'
              f'Площадь фигуры: {self.square()}\n'
              + '\n'
              )



class kvadrat(rectangle.rectangle):
    def __init__(self, length = 10, color_ = 'Красный'):
        rectangle.rectangle.__init__(self, length=length, color_=color_)
        self.__name = 'Квадрат'

    def square(self):
        res = self._length ** 2
        return res

    def get_name(self):
        return self.__name

    def get_color(self):
        return self._color

    def repr(self):
        print(f'Название фигуры: {self.get_name()}\n'
              f'Цвет фигуры: {self.get_color()}\n'
              f'Площадь фигуры: {self.square()}\n'
              + '\n'
              )
        

if __name__ == "__main__":
    circle_ = circle.circle(15, 'Синий')
    circle_.repr()
    kvadrat_ = kvadrat.kvadrat(10, 'Желтый')
    kvadrat_.repr()
    rectangle_ = rectangle.rectangle(5, 20, 'Черный')
    rectangle_.repr()