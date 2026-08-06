from town_example.town import Town


def main():
    t = Town("Dresden", 556780)
    print(str(t))
    print(repr(t))
    print(t.name)
    print(t.residents)


if __name__ == "__main__":
    main()
