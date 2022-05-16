import numpy as np
import mediapipe as mp
import cv2

IMG_HEIGH = 150
IMG_WIDTH = 150

class hand_detector():
    def __init__(self, mode = False, max_hands = 1, detection_con = 0.5, track_con = 0.5) -> None:
        self._mode = mode
        self._maxHands = max_hands
        self._detectionCon = detection_con
        self._trackCon = track_con

        self._mpHands = mp.solutions.hands
        self._hands = self._mpHands.Hands(static_image_mode = self._mode,
                                         max_num_hands = self._maxHands, 
                                         min_detection_confidence = self._detectionCon, 
                                         min_tracking_confidence = self._trackCon)

        self._capture = cv2.VideoCapture(0)

    def get_landmarks(self):
        success, img = self._capture.read()

        self._results = self._hands.process(img)
        return self._results.multi_hand_landmarks

    def draw_landmarks(self, img = 0, handNo = 0, draw = True):
        self.get_landmarks()

        if img == 0:
            img = np.zeros((IMG_HEIGH, IMG_WIDTH, 3), np.uint8)

        if self._results.multi_hand_landmarks:
            hand = self._results.multi_hand_landmarks[handNo]
            i = 0

            for id, lm in enumerate(hand.landmark):
                h, w, c = img.shape
                cx, cy = int(lm.x * w), int(lm.y * h)

                if draw:
                    cv2.circle(img, (cx, cy), 3, (255 - i, 0 + i, 255 - i), cv2.FILLED)
                    i += 20

        img.flags.writeable = False # Make it a constant reference for performance purposes
        return img

    def print_landmarks(self):
        view = self.draw_landmarks()
        cv2.imshow("Landmarks", view)
