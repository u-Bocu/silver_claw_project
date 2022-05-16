import cv2
import hand_landmarks_detector as hand_landmarks_detector

hd = hand_landmarks_detector.hand_detector()

while True:
    hd.print_landmarks()
    if cv2.waitKey(1) & 0xFF == ord('q'):
        break