import pyautogui
import pygame
import PIL

screeni = pyautogui.screenshot()

screen = pygame.display.set_mode((1920, 1080))

scroll = 1

def pilToSurface(img):
    return pygame.image.fromstring(img.tobytes(), img.size, img.mode)

while True:
    mouse = pyautogui.position()

    image = screeni.crop(box=(mouse[0] - 1920/2/scroll, mouse[1] - 1080/2/scroll, mouse[0] + 1920/2/scroll, mouse[1] + 1080/2/scroll))
    image = pilToSurface(image)
    image = pygame.transform.scale(image, (1920, 1080))
    screen.blit(image, (0, 0))
    for event in pygame.event.get():
        if event.type == pygame.MOUSEWHEEL:
            scroll += scroll * event.dict['y'] * 0.1
            if scroll < 0.9: scroll = 0.9
        if event.type == pygame.QUIT:
            quit()
    pygame.display.flip()
