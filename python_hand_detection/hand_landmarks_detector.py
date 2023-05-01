import mediapipe as mp
import cv2

class hand_detector():
    def __init__(self, mode = False, max_hands = 2, detection_con = 0.5, track_con = 0.5) -> None:
        self._mode = mode
        self._maxHands = max_hands
        self._detectionCon = detection_con
        self._trackCon = track_con

        self._mpHands = mp.solutions.hands
        self._hands = self._mpHands.Hands(static_image_mode = self._mode,
                                         max_num_hands = self._maxHands, 
                                         model_complexity = 0,
                                         min_detection_confidence = self._detectionCon, 
                                         min_tracking_confidence = self._trackCon)

        self._capture = cv2.VideoCapture(0)

    def get_landmarks(self, handNo = 0):
        _success, img = self._capture.read()
        imgRGB = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

        self._results = self._hands.process(imgRGB)

        landmarks = None
        if self._results.multi_hand_landmarks and len(self._results.multi_hand_landmarks) > handNo:
            hand = self._results.multi_hand_landmarks[handNo]
            landmarks = []

            for _id, lm in enumerate(hand.landmark):
                landmarks.append((lm.x, lm.y, lm.z))

        return landmarks
