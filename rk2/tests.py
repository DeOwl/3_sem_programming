import unittest
from main import *

class TestJoinOneToMany(unittest.TestCase):
    def test_one_to_many(self):
        result = OneToMany(secs, docs)
        self.assertTrue(len(result) == 10)


class TestJoinManyToMAny(unittest.TestCase):
    def test_many_to_many(self):
        result = ManyToMany(secs, docs, secs_docs)
        self.assertTrue(len(result) == 10)


class TestResults(unittest.TestCase):
    def test_get_a1_result(self):
        data = [("Секция 1", 5, "Договор 1"), ("А Секция 2", 5, "Договор 1"), ("Секция 3", 5, "Договор 2")]
        result = get_first(data)
        self.assertEqual(result, [("А Секция 2", 5, "Договор 1")])  # Проверка корректности результата

    def test_get_a1_empty_result(self):
        data = []  # пустой список
        result = get_first(data)
        self.assertEqual(result, [])  # ожидаемый результат - пустой список

    def test_get_a2_result(self):
        data = [("Секция 1", 5, "Договор 1"), ("А Секция 2", 6, "Договор 1"), ("Секция 3", 5, "Договор 2")]
        result = get_second(data)
        self.assertEqual(result, [("Секция 1", 5, "Договор 1"), ("Секция 3", 5, "Договор 2")])  # Проверка корректности результата

    def test_get_a3_result(self):
        data = [("Секция 1", 5, "Договор 1"), 
                ("А Секция 2", 6, "Договор 1"), 
                ("Секция 3", 5, "Договор 2")
        ]
        result = sorted(data, key=lambda x: x[0])  # ожидаемый результат - отсортированный по имени библиотеки
        self.assertEqual(result, get_third(data))

if __name__ == '__main__':
    unittest.main()